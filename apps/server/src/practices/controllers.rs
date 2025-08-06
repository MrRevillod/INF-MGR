use sword::prelude::*;
use uuid::Uuid;

use crate::{container::AppModule, practices::PracticeService};

#[controller("/practices")]
pub struct PracticesController {}

#[routes]
impl PracticesController {
    #[post("/{id}/approve")]
    async fn approve_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let service = ctx.get_dependency::<AppModule, dyn PracticeService>()?;

        let Some(practice) = service.get_by_id(&id).await? else {
            return Err(HttpResponse::NotFound());
        };

        if practice.is_approved {
            return Err(HttpResponse::BadRequest());
        }

        service.approve(&id).await?;

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
