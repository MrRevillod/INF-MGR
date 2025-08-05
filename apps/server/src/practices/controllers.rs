use sword::prelude::*;

#[controller("/practices")]
pub struct PracticesController {}

#[routes]
impl PracticesController {
    #[post("/{id}/approve/{token}")]
    async fn approve_practice(_: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/reject/{token}")]
    async fn reject_practice(_: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/evaluate/{token}")]
    async fn evaluate_practice(_: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }
}
