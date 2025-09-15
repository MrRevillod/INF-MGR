use sword::__internal::IntoResponse;
use sword::prelude::*;
use tokio::fs;
use uuid::Uuid;

use crate::{
    config::ServerConfig,
    practices::{
        CreatePracticeDto, EvaluatePracticeDto, PracticeService, PracticeStatus,
        UpdatePracticeDto,
    },
    shared::di::AppModule,
};

#[controller("/enrollments")]
pub struct EnrollmentsController {}

#[routes]
impl EnrollmentsController {
    #[post("/{id}/practice")]
    async fn create_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let dto = ctx.validated_body::<CreatePracticeDto>()?;

        let practice = ctx
            .di::<AppModule, dyn PracticeService>()?
            .create(&enrollment_id, dto)
            .await?;

        Ok(HttpResponse::Created().data(practice))
    }

    #[post("/{id}/practice/{practice_id}/approve")]
    async fn approve_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let practice_id = ctx.param::<Uuid>("practice_id")?;

        let service = ctx.di::<AppModule, dyn PracticeService>()?;

        let Some(practice) = service.get_by_id(&practice_id).await? else {
            return Err(HttpResponse::NotFound());
        };

        if practice.practice_status != PracticeStatus::Pending {
            return Err(HttpResponse::BadRequest());
        }

        service
            .update_status(&enrollment_id, &practice_id, PracticeStatus::Approved)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/practice/{practice_id}/decline")]
    async fn decline_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let practice_id = ctx.param::<Uuid>("practice_id")?;

        let service = ctx.di::<AppModule, dyn PracticeService>()?;

        let Some(practice) = service.get_by_id(&practice_id).await? else {
            return Err(HttpResponse::NotFound());
        };

        if practice.practice_status != PracticeStatus::Pending {
            return Err(HttpResponse::BadRequest());
        }

        service
            .update_status(&enrollment_id, &practice_id, PracticeStatus::Declined)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/practice/{practice_id}/authorize")]
    async fn authorize_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let mut form_data = ctx.multipart().await?;

        while let Some(field) = form_data.next_field().await.ok().flatten() {
            if field.name() != Some("auth_doc".into()) {
                return Err(HttpResponse::BadRequest());
            }

            let service = ctx.di::<AppModule, dyn PracticeService>()?;
            let field_bytes = field
                .bytes()
                .await
                .map_err(|_| HttpResponse::BadRequest())?;

            service
                .authorize(&practice_id, field_bytes.to_vec())
                .await?;
        }

        Ok(HttpResponse::Ok())
    }

    #[get("/practice/{practice_id}/docs")]
    async fn get_practice_docs(
        ctx: Context,
    ) -> Result<impl IntoResponse, HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let documents_dir = ctx.config::<ServerConfig>()?.documents_dir;

        let file_path = format!(
            "{}/practices/{}/authorization.pdf",
            documents_dir, practice_id
        );

        let buff = fs::read(&file_path).await.map_err(|e| {
            tracing::error!("Failed to open/read file {}: {e}", file_path);
            HttpResponse::NotFound()
        })?;

        Ok((StatusCode::OK, [("Content-Type", "application/pdf")], buff))
    }

    #[post("/{id}/practice/{practice_id}/evaluate/{evaluation_id}")]
    async fn evaluate_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let evaluation_id = ctx.param::<Uuid>("evaluation_id")?;

        let dto = ctx.validated_body::<EvaluatePracticeDto>()?;

        let practice_service = ctx.di::<AppModule, dyn PracticeService>()?;

        practice_service
            .evaluate(&enrollment_id, &practice_id, &evaluation_id, dto)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[patch("/{id}/practice")]
    async fn update_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let dto = ctx.validated_body::<UpdatePracticeDto>()?;

        let service = ctx.di::<AppModule, dyn PracticeService>()?;
        let practice = service.update(&enrollment_id, dto).await?;

        Ok(HttpResponse::Ok().data(practice))
    }

    #[delete("/practice/{practice_id}")]
    async fn delete_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let service = ctx.di::<AppModule, dyn PracticeService>()?;

        service.remove(&practice_id).await?;
        Ok(HttpResponse::NoContent())
    }
}
