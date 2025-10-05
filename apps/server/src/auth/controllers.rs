use crate::{
    auth::{Authentication, CallBackParams, OAuthService, services::SessionService},
    config::AuthConfig,
    shared::{di::AppModule, infrastructure::http::ContextExt},
};

use serde_json::json;
use sword::prelude::*;

#[controller("/auth")]
pub struct AuthController;

#[routes]
impl AuthController {
    #[post("/login")]
    async fn login(ctx: Context) -> HttpResult<HttpResponse> {
        let service = ctx.di::<AppModule, dyn OAuthService>()?;
        let login_data = service.oauth_login().await?;

        Ok(HttpResponse::Ok().data(login_data))
    }

    #[get("/callback")]
    async fn auth_callback(ctx: Context) -> HttpResult<HttpResponse> {
        let callback_params = ctx.query::<CallBackParams>()?.ok_or(
            HttpResponse::BadRequest()
                .message("Faltan parámetros de callback de Google OAuth"),
        )?;

        let (user, oauth_token) = ctx
            .di::<AppModule, dyn OAuthService>()?
            .validate_callback(callback_params)
            .await?;

        let auth_config = ctx.config::<AuthConfig>()?;

        let session = ctx
            .di::<AppModule, dyn SessionService>()?
            .create_session(&user, oauth_token, auth_config.session_ttl_seconds)
            .await?;

        Ok(HttpResponse::Ok().data(json!({
            "token_type": "Bearer",
            "access_token": session.access_token,
            "refresh_token": session.refresh_token,
        })))
    }

    #[post("/refresh")]
    async fn refresh_token(ctx: Context) -> HttpResult<HttpResponse> {
        let auth_config = ctx.config::<AuthConfig>()?;

        // Use the ContextExt method to extract Bearer tokens
        let (_access_token, refresh_token) = ctx.get_bearer_tokens()?;

        let session = ctx
            .di::<AppModule, dyn SessionService>()?
            .refresh_session(refresh_token, auth_config.session_ttl_seconds)
            .await?;

        // Return new tokens in the same format
        Ok(HttpResponse::Ok().data(json!({
            "access_token": session.access_token,
            "refresh_token": session.refresh_token,
            "token_type": "Bearer"
        })))
    }

    #[post("/logout")]
    #[middleware(Authentication)]
    async fn logout(ctx: Context) -> HttpResult<HttpResponse> {
        // Use the ContextExt method to extract Bearer tokens
        let (access_token, _refresh_token) = ctx.get_bearer_tokens()?;

        ctx.di::<AppModule, dyn SessionService>()?
            .close_session(&access_token)
            .await?;

        Ok(HttpResponse::Ok().data(json!({
            "message": "Session closed successfully"
        })))
    }
}
