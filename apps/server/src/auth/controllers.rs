use sword::prelude::*;

use crate::{auth::service::OAuthService, shared::di::AppModule};

#[controller("/auth")]
pub struct AuthController;

#[routes]
impl AuthController {
    #[post("/login")]
    async fn login(ctx: Context) -> HttpResult<HttpResponse> {
        let oauth_service = ctx.di::<AppModule, dyn OAuthService>()?;

        Ok(HttpResponse::Ok())
    }

    #[get("/callback")]
    async fn auth_callback() -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }
}
