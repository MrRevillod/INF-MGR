use sword::prelude::*;

#[controller("/auth")]
pub struct AuthController;

#[routes]
impl AuthController {
    #[post("/login")]
    async fn login() -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }

    #[get("/google_callback")]
    async fn auth_callback() -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }
}
