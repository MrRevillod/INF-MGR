use async_trait::async_trait;
use oauth2::{
    AuthorizationCode, CsrfToken, Scope, TokenResponse,
    reqwest::{self, redirect::Policy},
};

use shaku::{Component, Interface};
use std::sync::Arc;

use crate::{
    auth::{
        dtos::{AuthSuccessParams, CallBackParams, GoogleUserInfo, LoginData},
        repository::AuthRepository,
    },
    shared::{AppError, AppResult, errors::AuthError, oauth::OAuthClient},
    user_filter,
    users::{User, UserFilter, UserRepository},
};

#[derive(Component)]
#[shaku(interface = OAuthService)]
pub struct GoogleOAuthService {
    #[shaku(inject)]
    oauth_client: Arc<dyn OAuthClient>,

    #[shaku(inject)]
    auth_repository: Arc<dyn AuthRepository>,

    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
pub trait OAuthService: Interface {
    async fn oauth_login(&self) -> AppResult<LoginData>;
    async fn validate_callback(&self, params: CallBackParams) -> AppResult<User>;
    async fn get_user_info(&self, access_token: &str) -> AppResult<GoogleUserInfo>;
}

#[async_trait]
impl OAuthService for GoogleOAuthService {
    async fn oauth_login(&self) -> AppResult<LoginData> {
        let client = self.oauth_client.get_client();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".into()))
            .add_scope(Scope::new("profile".into()))
            .add_scope(Scope::new("profile".into()))
            .url();

        self.auth_repository.save_csrf_token(csrf_token.secret()).await?;

        Ok(LoginData {
            auth_url: auth_url.to_string(),
        })
    }

    async fn get_user_info(&self, access_token: &str) -> AppResult<GoogleUserInfo> {
        let http_client = self.oauth_client.get_http_client();

        let response = http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        if !response.status().is_success() {
            return Err(AppError::InternalServerError("Failed to fetch user info".into()));
        }

        let user_info: GoogleUserInfo =
            response.json().await.map_err(|e| AppError::InternalServerError(e.into()))?;

        Ok(user_info)
    }

    async fn validate_callback(&self, params: CallBackParams) -> AppResult<User> {
        let CallBackParams { success, error } = params;

        if let Some(err) = error {
            return Err(AuthError::OAuthError(err.error))?;
        }

        let Some(AuthSuccessParams { code, state }) = success else {
            return Err(AuthError::OAuthError("Missing success parameters".into()))?;
        };

        let client = self.oauth_client.get_client();

        if self.auth_repository.get_csrf_token(state.as_str()).await?.is_none() {
            return Err(AuthError::OAuthError("Invalid CSRF token".into()))?;
        }

        let http_client = reqwest::ClientBuilder::new()
            .redirect(Policy::none())
            .build()
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        let access_token = token_result.access_token().secret();
        let google_user_info = self.get_user_info(access_token).await?;

        let system_user = self
            .user_repository
            .find_one(user_filter! { email: google_user_info.email.clone() })
            .await?;

        let Some(mut user) = system_user else {
            return Err(AuthError::UserNotFound(google_user_info.email))?;
        };

        if user.google_id.is_none() {
            user.google_id = Some(google_user_info.id.clone());
            self.user_repository.save(user.clone()).await?;
        }

        Ok(user)
    }
}
