use std::sync::Arc;

use serde_json::json;
use sword::prelude::*;

use crate::{
    auth::{
        Authentication, CallBackParams, OAuthService,
        services::{CookieBuilder as Cookie, SessionService},
    },
    config::{AuthConfig, ServerConfig},
    shared::http::ContextExt,
};

#[controller("/auth")]
pub struct AuthController {
    auth_config: AuthConfig,
    server_config: ServerConfig,
    oauth_service: Arc<OAuthService>,
    session_service: Arc<SessionService>,
}

#[routes]
impl AuthController {
    #[post("/login")]
    async fn login(&self) -> HttpResult {
        let login_data = self.oauth_service.oauth_login().await?;
        Ok(HttpResponse::Ok().data(login_data))
    }

    #[get("/callback")]
    async fn auth_callback(&self, req: Request) -> HttpResult {
        let callback_params = req.query::<CallBackParams>()?.ok_or_else(|| {
            HttpResponse::BadRequest()
                .message("Faltan parámetros de callback de Google OAuth")
        })?;

        let (user, oauth_token) = self
            .oauth_service
            .validate_callback(callback_params)
            .await?;

        let session = self
            .session_service
            .create_session(&user, oauth_token, self.auth_config.session_ttl_seconds)
            .await?;

        let access_cookie = Cookie::new("ACCESS", session.access_token)
            .max_age(self.auth_config.access_exp_ms)
            .build();

        let refresh_cookie = Cookie::new("REFRESH", session.refresh_token)
            .max_age(self.auth_config.access_exp_ms)
            .build();

        let client_app_url = self.server_config.clone().client_app_url;

        req.cookies()?.add(access_cookie);
        req.cookies()?.add(refresh_cookie);

        Ok(HttpResponse::TemporaryRedirect(&format!(
            "{client_app_url}/auth/callback"
        )))
    }

    #[post("/refresh")]
    async fn refresh_token(&self, req: Request) -> HttpResult {
        let (_, refresh_token) = req.get_bearer_tokens()?;

        let session = self
            .session_service
            .refresh_session(refresh_token, self.auth_config.session_ttl_seconds)
            .await?;

        Ok(HttpResponse::Ok().data(json!({
            "access_token": session.access_token,
            "refresh_token": session.refresh_token,
            "token_type": "Bearer"
        })))
    }

    #[post("/logout")]
    #[uses(Authentication)]
    async fn logout(&self, req: Request) -> HttpResult {
        let (access_token, _) = req.get_bearer_tokens()?;

        self.session_service.close_session(&access_token).await?;

        Ok(HttpResponse::Ok().message("Session closed successfully"))
    }

    #[get("/me")]
    #[uses(Authentication)]
    async fn me(&self, req: Request) -> HttpResult {
        Ok(HttpResponse::Ok().data(req.get_current_user()?))
    }
}
