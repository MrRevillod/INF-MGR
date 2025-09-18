use sword::prelude::*;
use uuid::Uuid;

use crate::{
    auth::{Authentication, permissions::RequirePermission},
    courses::*,
    enrollments::*,
    shared::di::AppModule,
};

#[controller("/courses")]
#[middleware(Authentication)]
#[doc = "Controlador para la gestión de cursos y sus inscripciones\n"]
#[doc = "Requiere autenticación para todas las rutas"]
pub struct CoursesController {}

#[routes]
impl CoursesController {
    #[get("/")]
    #[middleware(RequirePermission, config = "courses:read")]
    #[doc = "Obtener todos los cursos registrados en el sistema\n"]
    #[doc = "Se transforma el output sumando al profesor del curso\n"]
    #[doc = "Requiere el permiso 'courses:read' exclusivo de administradores y secretaría"]
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

    #[get("/{teacher_id}")]
    #[middleware(RequirePermission, config = "courses:read:own")]
    #[doc = "Obtener todos los cursos de un profesor específico\n"]
    #[doc = "Se transforma el output sumando al profesor del curso\n"]
    async fn get_courses_by_teacher(ctx: Context) -> HttpResult<HttpResponse> {
        let teacher_id = ctx.param::<Uuid>("teacher_id")?;
        let service = ctx.di::<AppModule, dyn CourseService>()?;

        let courses = service
            .get_all(course_filter! { teacher_id })
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(courses))
    }

    #[get("/{id}/students")]
    #[middleware(RequirePermission, config = "enrollments:read:related")]
    #[doc = "Obtener todos los estudiantes inscritos en un curso específico\n"]
    #[doc = "Se transforma el output sumando la información del estudiante y su práctica (si aplica)\n"]
    #[doc = "Requiere que el usuario autenticado sea el profesor o superiores"]
    async fn get_course_enrollments(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("id")?;
        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;

        let enrollments = service
            .get_all(enrollment_filter! { course_id })
            .await?
            .into_iter()
            .map(EnrollmentResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(enrollments))
    }

    #[post("/")]
    #[middleware(RequirePermission, config = "courses:create")]
    #[doc = "Crear un nuevo curso en el sistema"]
    #[doc = "Requiere el permiso 'courses:create' exclusivo de administradores y secretaría"]
    async fn create_course(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateCourseDto>()?;
        let service = ctx.di::<AppModule, dyn CourseService>()?;

        let asignature = service.create(input).await?;

        Ok(HttpResponse::Created().data(asignature))
    }

    #[post("/enroll")]
    #[middleware(RequirePermission, config = "enrollments:create")]
    #[doc = "Inscribir un estudiante en un curso"]
    #[doc = "Requiere el permiso 'enrollments:create' exclusivo de administradores y secretaría"]
    async fn create_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateEnrollmentDto>()?;
        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;

        let enrollment = service.create(input).await?;

        Ok(HttpResponse::Created().data(enrollment))
    }

    #[patch("/{id}")]
    #[middleware(RequirePermission, config = "courses:update:own")]
    #[doc = "Actualizar la información de un curso"]
    #[doc = "Requiere que el usuario autenticado sea el profesor del curso o superiores"]
    async fn update_course(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateCourseDto>()?;

        let service = ctx.di::<AppModule, dyn CourseService>()?;
        let updated_asignature = service.update(&asignature_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_asignature))
    }

    #[patch("/enrollments/{enrollment_id}")]
    #[middleware(RequirePermission, config = "enrollments:update:related")]
    #[doc = "Actualizar la información de una inscripción"]
    #[doc = "Requiere el permiso 'enrollments:update:related' exclusivo de profesores y superiores"]
    async fn update_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("enrollment_id")?;
        let input = ctx.validated_body::<UpdateEnrollmentDto>()?;

        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;
        let updated_enrollment = service.update(&enrollment_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_enrollment))
    }

    #[delete("/{id}")]
    #[middleware(RequirePermission, config = "courses:delete")]
    #[doc = "Eliminar un curso del sistema (solo si no tiene estudiantes inscritos)"]
    #[doc = "Requiere el permiso 'courses:delete' exclusivo de administradores"]
    async fn remove(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("id")?;
        let service = ctx.di::<AppModule, dyn CourseService>()?;

        service.remove(&course_id).await?;

        Ok(HttpResponse::Ok())
    }

    #[delete("/enrollments/{enrollment_id}")]
    #[middleware(RequirePermission, config = "enrollments:delete")]
    #[doc = "Eliminar una inscripción de un estudiante en un curso"]
    #[doc = "Requiere el permiso 'enrollments:delete' exclusivo de administradores y secretaría"]
    async fn remove_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("enrollment_id")?;
        let service = ctx.di::<AppModule, dyn EnrollmentService>()?;

        service.remove(&enrollment_id).await?;

        Ok(HttpResponse::Ok())
    }
}
