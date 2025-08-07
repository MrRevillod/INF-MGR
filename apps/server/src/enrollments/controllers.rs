use sword::prelude::*;
use uuid::Uuid;

use crate::{
    container::AppModule,
    practices::{CreatePracticeDto, PracticeService, UpdatePracticeDto},
};

#[controller("/enrollments")]
pub struct EnrollmentsController {}

#[routes]
impl EnrollmentsController {
    #[post("/{id}/practice")]
    async fn create_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let dto = ctx.validated_body::<CreatePracticeDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn PracticeService>()?;

        let practice = service.create(&enrollment_id, dto).await?;
        Ok(HttpResponse::Created().data(practice))
    }

    #[post("/{id}/practice/{practice_id}/approve")]
    async fn approve_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let practice_id = ctx.param::<Uuid>("practice_id")?;

        let service = ctx.get_dependency::<AppModule, dyn PracticeService>()?;

        let Some(practice) = service.get_by_id(&practice_id).await? else {
            return Err(HttpResponse::NotFound());
        };

        if practice.is_approved {
            return Err(HttpResponse::BadRequest());
        }

        service.approve(&enrollment_id, &practice_id).await?;

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

    #[patch("/{id}/practice")]
    async fn update_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let dto = ctx.validated_body::<UpdatePracticeDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn PracticeService>()?;
        let practice = service.update(&enrollment_id, dto).await?;

        Ok(HttpResponse::Ok().data(practice))
    }

    #[delete("/practice/{practice_id}")]
    async fn delete_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let service = ctx.get_dependency::<AppModule, dyn PracticeService>()?;

        service.remove(&practice_id).await?;
        Ok(HttpResponse::NoContent())
    }
}
