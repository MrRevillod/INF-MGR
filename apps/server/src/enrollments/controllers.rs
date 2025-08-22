use std::io::Read;

use axum::{http::StatusCode, response::IntoResponse};
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

    #[post("/{id}/practice/{practice_id}/reject")]
    async fn reject_practice(_: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/practice/{practice_id}/authorize")]
    async fn authorize_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let form_data = ctx.multipart().await?;

        let Some(field) = form_data.first() else {
            return Err(HttpResponse::BadRequest());
        };

        if field.name != Some("auth_doc".into()) {
            return Err(HttpResponse::BadRequest());
        }

        let service = ctx.get_dependency::<AppModule, dyn PracticeService>()?;

        service.authorize(&practice_id, field.data.bytes()).await?;

        Ok(HttpResponse::Ok())
    }

    #[get("/practice/{practice_id}/docs")]
    async fn get_practice_docs(ctx: Context) -> Result<impl IntoResponse, HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;

        let file_path = format!(
            "{}/practices/{}/authorization.pdf",
            std::env::var("DOCUMENTS_DIR").unwrap_or(".".to_string()),
            practice_id
        );

        let buff = tokio::fs::read(&file_path).await.map_err(|e| {
            tracing::error!("Failed to open/read file {}: {e}", file_path);
            HttpResponse::NotFound()
        })?;

        Ok((StatusCode::OK, [("Content-Type", "application/pdf")], buff))
    }

    #[post("/{id}/practice/{practice_id}/evaluate")]
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
