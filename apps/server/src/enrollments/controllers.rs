use sword::prelude::*;
use tokio::fs;
use uuid::Uuid;

use crate::{
    auth::{Authentication, MinimumRequiredRole},
    config::ServerConfig,
    courses::CourseService,
    enrollments::EnrollmentService,
    practices::*,
    shared::{AppModule, ContextExt, FileResponse},
};

#[controller("/enrollments")]
pub struct EnrollmentsController {}

#[routes]
impl EnrollmentsController {
    #[post("/{id}/practice")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    #[doc = "Crear una práctica para una inscripción específica"]
    async fn create_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let dto = ctx.validated_body::<CreatePracticeDto>()?;

        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required {
            let enrollment_service = ctx.di::<AppModule, dyn EnrollmentService>()?;

            let (enrollment, _, _) =
                enrollment_service.get_by_id(&enrollment_id).await?;

            let course_service = ctx.di::<AppModule, dyn CourseService>()?;

            let (course, _) =
                course_service.get_by_id(&enrollment.course_id).await?;

            if course.teacher_id != owner_validation.user_id {
                return Err(HttpResponse::Forbidden().message(
                    "No tienes permiso para crear prácticas en esta inscripción",
                ));
            }
        }

        let practice = ctx
            .di::<AppModule, dyn PracticeService>()?
            .create(&enrollment_id, dto)
            .await?;

        Ok(HttpResponse::Created().data(practice))
    }

    #[post("/{id}/practice/{practice_id}/approve")]
    #[doc = "Aprovar práctica por el supervisor en la empresa."]
    #[doc = "Este endpoint no requiere autenticación ya que se asume que el supervisor no es usuario del sistema."]
    #[doc = "Solo se pude aprobar la práctica si su estado es 'Pending' (En espera de aprobación)."]
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
    #[doc = "Rechazar práctica por el supervisor en la empresa."]
    #[doc = "Este endpoint no requiere autenticación ya que se asume que el supervisor no es usuario del sistema."]
    #[doc = "Solo se pude rechazar la práctica si su estado es 'Pending' (En espera de aprobación)."]
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
    #[doc = "Subir documento de autorización de una práctica"]
    #[doc = "Este endpoint no requiere autenticación ya que se asume que el supervisor no es usuario del sistema."]
    #[doc = "Se establece que esta acción es realizable una única vez por práctica."]
    async fn authorize_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let mut form_data = ctx.multipart().await?;

        while let Some(field) = form_data.next_field().await.ok().flatten() {
            if field.name() != Some("auth_doc") {
                return Err(HttpResponse::BadRequest());
            }

            let service = ctx.di::<AppModule, dyn PracticeService>()?;
            let field_bytes = field
                .bytes()
                .await
                .map_err(|_| HttpResponse::BadRequest())?;

            service
                .authorize(&enrollment_id, field_bytes.to_vec())
                .await?;
        }

        Ok(HttpResponse::Ok())
    }

    #[get("/practice/{practice_id}/docs")]
    #[middleware(Authentication)]
    #[doc = "Obtener el documento de autorización de una práctica"]
    async fn get_practice_docs(ctx: Context) -> HttpResult<FileResponse> {
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

        Ok(FileResponse::Document(buff))
    }

    #[post("/{id}/practice/{practice_id}/evaluate/{evaluation_id}")]
    #[doc = "Evaluar práctica por el supervisor en la empresa."]
    #[doc = "Este endpoint no requiere autenticación ya que se asume que el supervisor no es usuario del sistema."]
    #[doc = "Solo se pude evaluar la práctica si su estado es 'Approved' (Aprobada)."]
    async fn evualuate_from_enterprise(ctx: Context) -> HttpResult<HttpResponse> {
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

    #[post("/{id}/practice-report/upload")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "student")]
    #[doc = "Subir informe de práctica por el estudiante."]
    async fn upload_practice_report(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let mut form_data = ctx.multipart().await?;

        while let Some(field) = form_data.next_field().await.ok().flatten() {
            if field.name() != Some("report") {
                return Err(HttpResponse::BadRequest());
            }

            let service = ctx.di::<AppModule, dyn EnrollmentService>()?;
            let field_bytes = field
                .bytes()
                .await
                .map_err(|_| HttpResponse::BadRequest())?;

            service
                .upload_final_report(&enrollment_id, field_bytes.to_vec())
                .await?;
        }

        Ok(HttpResponse::Ok())
    }

    #[patch("/{id}/practice")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    #[doc = "Actualizar una práctica para una inscripción específica"]
    async fn update_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("id")?;
        let dto = ctx.validated_body::<UpdatePracticeDto>()?;

        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required {
            let enrollment_service = ctx.di::<AppModule, dyn EnrollmentService>()?;

            let (enrollment, _, _) =
                enrollment_service.get_by_id(&enrollment_id).await?;

            let course_service = ctx.di::<AppModule, dyn CourseService>()?;

            let (course, _) =
                course_service.get_by_id(&enrollment.course_id).await?;

            if course.teacher_id != owner_validation.user_id {
                return Err(HttpResponse::Forbidden().message(
                    "No tienes permiso para actualizar prácticas en esta inscripción",
                ));
            }
        }

        let service = ctx.di::<AppModule, dyn PracticeService>()?;
        let practice = service.update(&enrollment_id, dto).await?;

        Ok(HttpResponse::Ok().data(practice))
    }

    #[delete("/practice/{practice_id}")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "secretary")]
    #[doc = "Eliminar una práctica por su ID"]
    async fn delete_practice(ctx: Context) -> HttpResult<HttpResponse> {
        let practice_id = ctx.param::<Uuid>("practice_id")?;
        let service = ctx.di::<AppModule, dyn PracticeService>()?;

        service.remove(&practice_id).await?;

        Ok(HttpResponse::NoContent())
    }
}
