use crate::{
    auth::{dtos::CallBackParams, service::OAuthService},
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
        
        match service.oauth_login().await {
            Ok(login_data) => Ok(HttpResponse::Ok().data(login_data)),
            Err(e) => {
                tracing::error!("OAuth login failed: {:?}", e);
                Err(e.into())
            }
        }
    }

    #[get("/callback")]
    async fn auth_callback(ctx: Context) -> HttpResult<HttpResponse> {
        let callback_params = ctx
            .query::<CallBackParams>()?
            .ok_or_else(|| {
                HttpResponse::BadRequest()
                    .message("Faltan parámetros de callback de Google OAuth")
            })?;

        let service = ctx.di::<AppModule, dyn OAuthService>()?;
        
        match service.validate_callback(callback_params).await {
            Ok(user) => {
                tracing::info!("User {} successfully authenticated via OAuth", user.email);
                Ok(HttpResponse::Ok().data(user))
            },
            Err(e) => {
                tracing::warn!("OAuth callback validation failed: {:?}", e);
                Err(e.into())
            }
        }
    }
}
