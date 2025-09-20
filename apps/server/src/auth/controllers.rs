use crate::{
    auth::{
        Authentication, CallBackParams, OAuthService,
        services::{CookieBuilder, SessionService},
    },
    config::{AuthConfig, ServerConfig},
    shared::di::AppModule,
};

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
    async fn auth_callback(mut ctx: Context) -> HttpResult<HttpResponse> {
        let callback_params = ctx.query::<CallBackParams>()?.ok_or(
            HttpResponse::BadRequest()
                .message("Faltan parámetros de callback de Google OAuth"),
        )?;

        let (user, oauth_token) = ctx
            .di::<AppModule, dyn OAuthService>()?
            .validate_callback(callback_params)
            .await?;

        let app_config = ctx.config::<ServerConfig>()?;
        let auth_config = ctx.config::<AuthConfig>()?;

        let session = ctx
            .di::<AppModule, dyn SessionService>()?
            .create_session(&user, oauth_token, auth_config.session_ttl_seconds)
            .await?;

        let access_cookie =
            CookieBuilder::new(("access", session.access_token.clone()))
                .max_age(auth_config.access_exp_ms)
                .build();

        let refresh_cookie =
            CookieBuilder::new(("refresh", session.refresh_token.clone()))
                .max_age(auth_config.refresh_exp_ms)
                .build();

        ctx.cookies_mut()?.add(access_cookie);
        ctx.cookies_mut()?.add(refresh_cookie);

        Ok(HttpResponse::TemporaryRedirect().add_header(
            "Location",
            &format!("{}/dashboard", app_config.client_app_url),
        ))
    }

    #[post("/refresh")]
    async fn refresh_token(mut ctx: Context) -> HttpResult<HttpResponse> {
        let cookies = ctx.cookies()?;
        let auth_config = ctx.config::<AuthConfig>()?;

        let refresh_token = cookies
            .get("refresh")
            .map(|c| c.value().to_string())
            .ok_or(HttpResponse::Unauthorized())?;

        let session = ctx
            .di::<AppModule, dyn SessionService>()?
            .refresh_session(
                refresh_token.to_string(),
                auth_config.session_ttl_seconds,
            )
            .await?;

        let access_cookie =
            CookieBuilder::new(("access", session.access_token.clone()))
                .max_age(auth_config.access_exp_ms)
                .build();

        let refresh_cookie =
            CookieBuilder::new(("refresh", session.refresh_token.clone()))
                .max_age(auth_config.refresh_exp_ms)
                .build();

        ctx.cookies_mut()?.add(access_cookie);
        ctx.cookies_mut()?.add(refresh_cookie);

        Ok(HttpResponse::Ok())
    }

    #[post("/logout")]
    #[middleware(Authentication)]
    async fn logout(mut ctx: Context) -> HttpResult<HttpResponse> {
        let cookies = ctx.cookies()?;
        let access_token = cookies.get("access").map(|c| c.value().to_string());

        if let Some(token) = access_token {
            ctx.di::<AppModule, dyn SessionService>()?
                .close_session(&token)
                .await?;
        }

        let expired_access_cookie =
            CookieBuilder::new(("access", String::default()))
                .max_age(0)
                .build();

        let expired_refresh_cookie =
            CookieBuilder::new(("refresh", String::default()))
                .max_age(0)
                .build();

        ctx.cookies_mut()?.add(expired_access_cookie);
        ctx.cookies_mut()?.add(expired_refresh_cookie);

        Ok(HttpResponse::Ok())
    }
}
