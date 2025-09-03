use crate::{
    auth::{CallBackParams, OAuthService, services::SessionService},
    config::ApplicationConfig,
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
        let callback_params = ctx.query::<CallBackParams>()?.ok_or_else(|| {
            HttpResponse::BadRequest().message("Faltan parámetros de callback de Google OAuth")
        })?;

        let oauth_service = ctx.di::<AppModule, dyn OAuthService>()?;
        let (user, oauth_token) = oauth_service.validate_callback(callback_params).await?;

        let app_config = ctx.config::<ApplicationConfig>()?;
        let redirect_url = format!("{}/dashboard", app_config.client_app_url);

        let session_service = ctx.di::<AppModule, dyn SessionService>()?;
        let session = session_service.create_session(&user, oauth_token).await?;

        let access_token_cookie = Cookie::build(("access_token", session.access_token.clone()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .build();

        let refresh_token_cookie = Cookie::build(("refresh_token", session.refresh_token.clone()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .build();

        ctx.cookies_mut()?.add(access_token_cookie);
        ctx.cookies_mut()?.add(refresh_token_cookie);

        Ok(HttpResponse::TemporaryRedirect().add_header("Location", &redirect_url))
    }
}
