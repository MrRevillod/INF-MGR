use serde_json::json;
use sword::prelude::*;

use crate::{
    auth::{
        Authentication, CallBackParams, OAuthService,
        services::{CookieBuilder as Cookie, SessionService},
    },
    config::{AuthConfig, ServerConfig},
    shared::{di::AppModule, infrastructure::http::ContextExt},
};

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

        let cookie_exp = ctx.config::<AuthConfig>()?.access_exp_ms;
        let client_app_url = ctx.config::<ServerConfig>()?.client_app_url;

        ctx.cookies()?.add(
            Cookie::new("ACCESS", session.access_token)
                .max_age(cookie_exp)
                .build(),
        );

        ctx.cookies()?.add(
            Cookie::new("REFRESH", session.refresh_token)
                .max_age(cookie_exp)
                .build(),
        );

        Ok(HttpResponse::TemporaryRedirect()
            .add_header("Location", &format!("{client_app_url}/auth/callback")))
    }

    #[post("/refresh")]
    async fn refresh_token(ctx: Context) -> HttpResult<HttpResponse> {
        let auth_config = ctx.config::<AuthConfig>()?;

        let (_, refresh_token) = ctx.get_bearer_tokens()?;

        let session = ctx
            .di::<AppModule, dyn SessionService>()?
            .refresh_session(refresh_token, auth_config.session_ttl_seconds)
            .await?;

        Ok(HttpResponse::Ok().data(json!({
            "access_token": session.access_token,
            "refresh_token": session.refresh_token,
            "token_type": "Bearer"
        })))
    }

    #[post("/logout")]
    #[middleware(Authentication)]
    async fn logout(ctx: Context) -> HttpResult<HttpResponse> {
        let (access_token, _) = ctx.get_bearer_tokens()?;

        ctx.di::<AppModule, dyn SessionService>()?
            .close_session(&access_token)
            .await?;

        Ok(HttpResponse::Ok().message("Session closed successfully"))
    }

    #[get("/me")]
    #[middleware(Authentication)]
    async fn me(ctx: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok().data(ctx.get_current_user()?))
    }
}
