use sword::prelude::*;
use uuid::Uuid;

use crate::{
    auth::{Authentication, MinimumRequiredRole},
    courses::*,
    enrollments::*,
    shared::{AppModule, ContextExt},
};

#[controller("/courses")]
#[middleware(Authentication)]
#[doc = "Controlador para la gestión de cursos y sus inscripciones\n"]
#[doc = "Requiere autenticación para todas las rutas"]
pub struct CoursesController {}

#[routes]
impl CoursesController {
    #[get("/")]
    #[middleware(MinimumRequiredRole, config = "secretary")]
    #[doc = "Obtener todos los cursos registrados en el sistema\n"]
    #[doc = "Se transforma el output sumando al profesor del curso\n"]
    async fn get_courses(ctx: Context) -> HttpResult<HttpResponse> {
        let service = ctx.di::<AppModule, dyn CourseService>()?;
        let courses = service
            .get_all(course_filter! {})
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(courses))
    }

    #[get("/teacher/{teacher_id}")]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    #[doc = "Obtener todos los cursos de un profesor específico\n"]
    #[doc = "Se transforma el output sumando al profesor del curso\n"]
    async fn get_courses_by_teacher(ctx: Context) -> HttpResult<HttpResponse> {
        let teacher_id = ctx.param::<Uuid>("teacher_id")?;
        let service = ctx.di::<AppModule, dyn CourseService>()?;

        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required && owner_validation.user_id != teacher_id {
            return Err(HttpResponse::Forbidden()
                .message("No tienes permiso para ver los cursos de este profesor"));
        }

        let courses = service
            .get_all(course_filter! { teacher_id })
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(courses))
    }

    #[get("/{id}/students")]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    #[doc = "Obtener todos los estudiantes inscritos en un curso específico\n"]
    #[doc = "Se transforma el output sumando la información del estudiante y su práctica (si aplica)\n"]
    #[doc = "Requiere que el usuario autenticado sea el profesor o superiores"]
    async fn get_course_enrollments(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("id")?;
        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;

        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required {
            ctx.di::<AppModule, dyn CourseService>()?
                .check_is_teacher_course(&course_id, &owner_validation.user_id)
                .await?;
        }

        let enrollments = service
            .get_all(enrollment_filter! { course_id })
            .await?
            .into_iter()
            .map(EnrollmentResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(enrollments))
    }

    #[post("/")]
    #[middleware(MinimumRequiredRole, config = "secretary")]
    #[doc = "Crear un nuevo curso en el sistema"]
    async fn create_course(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateCourseDto>()?;
        let service = ctx.di::<AppModule, dyn CourseService>()?;

        let asignature = service.create(input).await?;

        Ok(HttpResponse::Created().data(asignature))
    }

    #[post("/enroll")]
    #[middleware(MinimumRequiredRole, config = "secretary")]
    #[doc = "Inscribir un estudiante en un curso"]
    async fn create_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateEnrollmentDto>()?;
        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;

        let enrollment = service.create(input).await?;

        Ok(HttpResponse::Created().data(enrollment))
    }

    #[patch("/{id}")]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    #[doc = "Actualizar la información de un curso"]
    #[doc = "Requiere que el usuario autenticado sea el profesor del curso o superiores"]
    async fn update_course(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateCourseDto>()?;

        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required {
            ctx.di::<AppModule, dyn CourseService>()?
                .check_is_teacher_course(&asignature_id, &owner_validation.user_id)
                .await?;
        }

        let service = ctx.di::<AppModule, dyn CourseService>()?;
        let updated_asignature = service.update(&asignature_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_asignature))
    }

    #[patch("/enrollments/{enrollment_id}")]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    #[doc = "Actualizar la información de una inscripción"]
    async fn update_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("enrollment_id")?;
        let input = ctx.validated_body::<UpdateEnrollmentDto>()?;

        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;
        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required {
            let (enrollment, _, _) = service.get_by_id(&enrollment_id).await?;

            let course_service = ctx.di::<AppModule, dyn CourseService>()?;
            let (_, teacher) =
                course_service.get_by_id(&enrollment.course_id).await?;

            if teacher.id != owner_validation.user_id {
                return Err(HttpResponse::Forbidden()
                    .message("No tienes permiso para modificar esta inscripción"));
            }
        }

        let updated_enrollment = service.update(&enrollment_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_enrollment))
    }

    #[delete("/{id}")]
    #[middleware(MinimumRequiredRole, config = "administrator")]
    #[doc = "Eliminar un curso del sistema (solo si no tiene estudiantes inscritos)"]
    async fn remove(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("id")?;
        let service = ctx.di::<AppModule, dyn CourseService>()?;

        service.remove(&course_id).await?;

        Ok(HttpResponse::Ok())
    }

    #[delete("/enrollments/{enrollment_id}")]
    #[middleware(MinimumRequiredRole, config = "secretary")]
    #[doc = "Eliminar una inscripción de un estudiante en un curso"]
    async fn remove_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("enrollment_id")?;
        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;

        service.remove(&enrollment_id).await?;

        Ok(HttpResponse::Ok())
    }
}
