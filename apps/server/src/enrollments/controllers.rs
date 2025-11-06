use std::sync::Arc;

use bytes::Bytes;
use sword::prelude::*;
use tokio::fs;
use tracing::error;
use uuid::Uuid;

use crate::{
    auth::{Authentication, MinimumRequiredRole},
    config::ServerConfig,
    courses::CourseService,
    enrollments::EnrollmentService,
    practices::*,
    shared::{
        FileValidationConfig, FileValidationService, RequestFiles, http::ContextExt,
    },
    users::Role,
};

#[controller("/enrollments")]
pub struct EnrollmentsController {
    courses: Arc<CourseService>,
    enrollments: Arc<EnrollmentService>,
    practices: Arc<PracticeService>,
    server_config: ServerConfig,
}

#[routes]
impl EnrollmentsController {
    #[post("/{id}/practice")]
    #[uses(Authentication)]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    async fn create_practice(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("id")?;
        let dto = req.body_validator::<CreatePracticeDto>()?;
        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            let (enrollment, _, _, _) =
                self.enrollments.get_by_id(&enrollment_id).await?;

            let (course, _) = self.courses.get_by_id(&enrollment.course_id).await?;

            if course.teacher_id != owner_validation.user_id {
                return Err(HttpResponse::Forbidden().message(
                    "No tienes permiso para crear prácticas en esta inscripción",
                ));
            }
        }

        let practice = self.practices.create(&enrollment_id, dto).await?;

        Ok(HttpResponse::Created().data(practice))
    }

    #[post("/{id}/practice/{practice_id}/approve")]
    async fn supervisor_approve_practice(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("id")?;
        let practice_id = req.param::<Uuid>("practice_id")?;

        let Some(practice) = self.practices.get_by_id(&practice_id).await? else {
            return Err(HttpResponse::NotFound());
        };

        if practice.practice_status != PracticeStatus::Pending {
            return Err(HttpResponse::BadRequest());
        }

        self.practices
            .update_status(&enrollment_id, &practice_id, PracticeStatus::Approved)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/practice/{practice_id}/decline")]
    async fn supervisor_decline_practice(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("id")?;
        let practice_id = req.param::<Uuid>("practice_id")?;

        let Some(practice) = self.practices.get_by_id(&practice_id).await? else {
            return Err(HttpResponse::NotFound());
        };

        if practice.practice_status != PracticeStatus::Pending {
            return Err(HttpResponse::BadRequest());
        }

        self.practices
            .update_status(&enrollment_id, &practice_id, PracticeStatus::Declined)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/practice/{practice_id}/authorize")]
    #[uses(FileValidationService, config = FileValidationConfig { kind: "pdf", name: "auth_doc" })]
    async fn supervisor_auth_practice(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("id")?;

        let auth_doc = req.extensions.get::<Bytes>().ok_or(
            HttpResponse::BadRequest().message("Missing required document"),
        )?;

        self.practices
            .authorize(&enrollment_id, auth_doc.to_vec())
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[get("/practice/{practice_id}/docs")]
    #[uses(Authentication)]
    #[doc = "Obtener el documento de autorización de una práctica"]
    async fn practice_docs(&self, req: Request) -> FileResult {
        let practice_id = req.param::<Uuid>("practice_id")?;

        let file_path = format!(
            "{}/practices/{}/authorization.pdf",
            self.server_config.documents_dir, practice_id
        );

        let buff = fs::read(&file_path).await.map_err(|e| {
            error!("Failed to open/read file {file_path}: {e}");
            HttpResponse::NotFound()
        })?;

        Ok(FileResponse::builder()
            .filename("authorization.pdf")
            .content_type("application/pdf")
            .bytes(buff))
    }

    #[post("/{id}/practice/{practice_id}/evaluate/{evaluation_id}")]
    async fn evualuate_from_enterprise(&self, req: Request) -> HttpResult {
        let practice_id = req.param::<Uuid>("practice_id")?;
        let enrollment_id = req.param::<Uuid>("id")?;
        let evaluation_id = req.param::<Uuid>("evaluation_id")?;

        let dto = req.body_validator::<EvaluatePracticeDto>()?;

        self.practices
            .evaluate(&enrollment_id, &practice_id, &evaluation_id, dto)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[post("/{id}/practice-report/upload")]
    #[uses(FileValidationService, config = FileValidationConfig { kind: "zip", name: "project" })]
    #[uses(FileValidationService, config = FileValidationConfig { kind: "pdf", name: "document" })]
    async fn upload_practice_report(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("id")?;

        let files = req.extensions.get::<RequestFiles>().ok_or(
            HttpResponse::BadRequest().message("Missing required report files"),
        )?;

        let report = files.get("document").ok_or(
            HttpResponse::BadRequest().message("Missing required PDF document"),
        )?;

        let zipped_project = files.get("project").ok_or(
            HttpResponse::BadRequest().message("Missing required ZIP project"),
        )?;

        let files_vec = (report.to_vec(), zipped_project.to_vec());

        self.enrollments
            .upload_final_report(&enrollment_id, files_vec)
            .await?;

        Ok(HttpResponse::Ok())
    }

    #[patch("/{id}/practice")]
    #[uses(Authentication)]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Actualizar una práctica para una inscripción específica"]
    async fn update_practice(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("id")?;
        let dto = req.body_validator::<UpdatePracticeDto>()?;

        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            let (enrollment, _, _, _) =
                self.enrollments.get_by_id(&enrollment_id).await?;

            let (course, _) = self.courses.get_by_id(&enrollment.course_id).await?;

            if course.teacher_id != owner_validation.user_id {
                return Err(HttpResponse::Forbidden().message(
                    "No tienes permiso para actualizar prácticas en esta inscripción",
                ));
            }
        }

        let practice = self.practices.update(&enrollment_id, dto).await?;

        Ok(HttpResponse::Ok().data(practice))
    }

    #[delete("/practice/{practice_id}")]
    #[uses(Authentication)]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    #[doc = "Eliminar una práctica por su ID"]
    async fn delete_practice(&self, req: Request) -> HttpResult {
        let practice_id = req.param::<Uuid>("practice_id")?;
        self.practices.remove(&practice_id).await?;

        Ok(HttpResponse::NoContent())
    }
}
