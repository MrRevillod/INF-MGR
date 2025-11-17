use oauth2::{AuthorizationCode, CsrfToken, RefreshToken, Scope, TokenResponse};

use std::sync::Arc;
use sword::core::injectable;

use crate::{
    auth::*,
    shared::{AppError, AppResult, AuthError, OAuthClient, OAuthTokenType},
    users::*,
};

#[injectable]
pub struct OAuthService {
    oauth_client: Arc<OAuthClient>,
    auth_repository: Arc<OAuthRepository>,
    user_repository: Arc<UserRepository>,
}

impl OAuthService {
    pub async fn oauth_login(&self) -> AppResult<String> {
        let client = self.oauth_client.get_client();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".into()))
            .add_scope(Scope::new("profile".into()))
            .add_scope(Scope::new("profile".into()))
            .url();

        self.auth_repository
            .save_csrf_token(csrf_token.secret())
            .await?;

        Ok(auth_url.to_string())
    }

    pub async fn get_user_info(&self, access_token: &str) -> AppResult<GoogleUserInfo> {
        let http_client = self.oauth_client.get_http_client();

        let user_info_response = http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        if !user_info_response.status().is_success() {
            return Err(AppError::InternalServerError(
                "Failed to fetch user info".into(),
            ));
        }

        let user_info: GoogleUserInfo = user_info_response
            .json()
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        Ok(user_info)
    }

    pub async fn validate_callback(
        &self,
        params: CallBackParams,
    ) -> AppResult<(User, OAuthTokenType)> {
        let CallBackParams { success, error } = params;

        if let Some(err) = error {
            return Err(AuthError::OAuthError(err.error))?;
        }

        let Some(AuthSuccessParams { code, state }) = success else {
            return Err(AuthError::OAuthError("Missing success parameters".into()))?;
        };

        let client = self.oauth_client.get_client();

        if self
            .auth_repository
            .get_csrf_token(state.as_str())
            .await?
            .is_none()
        {
            return Err(AuthError::OAuthError("Invalid CSRF token".into()))?;
        }

        let token_response = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(self.oauth_client.get_http_client())
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        let access_token = token_response.access_token().secret();
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

        Ok((user, token_response))
    }

    pub async fn refresh_oauth_token(
        &self,
        refresh_token: &str,
    ) -> AppResult<OAuthTokenType> {
        let client = self.oauth_client.get_client();
        let refresh_token = RefreshToken::new(refresh_token.to_string());

        let token_response = client
            .exchange_refresh_token(&refresh_token)
            .request_async(self.oauth_client.get_http_client())
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        Ok(token_response)
    }
}
