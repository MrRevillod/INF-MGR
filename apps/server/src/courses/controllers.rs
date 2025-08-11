use sword::prelude::*;
use uuid::Uuid;

use crate::{
    container::AppModule,
    courses::{CourseResponse, CourseService, CreateCourseDto, UpdateCourseDto},
    enrollments::{
        CreateEnrollmentDto, EnrollmentFilter, EnrollmentResponse,
        EnrollmentService, UpdateEnrollmentDto,
    },
};

#[controller("/courses")]
pub struct CoursesController {}

#[routes]
impl CoursesController {
    /// Obtener todos los cursos registrados en el sistema
    /// Se transforma el output sumando los miembros del staff a cada curso (teacher + coord)

    #[get("/")]
    async fn get_courses(ctx: Context) -> HttpResult<HttpResponse> {
        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;
        let asignatures = service
            .get_all()
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(asignatures))
    }

    /// Obtener todos las inscripciones de un curso, incluye la información
    /// de cada estudiante, sus notas y demás.

    #[get("/{id}/students")]
    async fn get_course_enrollments(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("id")?;
        let service = ctx.get_dependency::<AppModule, dyn EnrollmentService>()?;

        let filter = EnrollmentFilter {
            course_id: Some(course_id),
            ..Default::default()
        };

        let enrollments = service
            .get_all(filter)
            .await?
            .into_iter()
            .map(EnrollmentResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(enrollments))
    }

    /// Crear y registrar un nuevo curso en el sistema

    #[post("/")]
    async fn create_course(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateCourseDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;

        let asignature = service.create(input).await?;

        Ok(HttpResponse::Created().data(asignature))
    }

    /// Registrar un estudiante en un curso (enrollment)

    #[post("/enroll")]
    async fn create_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateEnrollmentDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn EnrollmentService>()?;

        let enrollment = service.create(input).await?;

        Ok(HttpResponse::Created().data(enrollment))
    }

    /// Actualizar la información de un curso
    /// Profesor, cordinador a cargo y/o estado del curso

    #[patch("/{id}")]
    async fn update_course(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateCourseDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;
        let updated_asignature = service.update(&asignature_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_asignature))
    }

    /// Actualizar la información de un estudiante inscrito en un curso
    /// Notas, práctica, etc.

    #[patch("/enrollments/{enrollment_id}")]
    async fn update_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("enrollment_id")?;
        let input = ctx.validated_body::<UpdateEnrollmentDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn EnrollmentService>()?;
        let updated_enrollment = service.update(&enrollment_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_enrollment))
    }

    /// Eliminar un curso del sistema, solo posible si no posee
    /// estudiantes inscritos en el.

    #[delete("/{id}")]
    async fn remove(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("id")?;
        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;

        service.remove(&course_id).await?;

        Ok(HttpResponse::Ok())
    }

    /// Eliminar inscripción de un estudiante en un curso.

    #[delete("/enrollments/{enrollment_id}")]
    async fn remove_enrollment(ctx: Context) -> HttpResult<HttpResponse> {
        let enrollment_id = ctx.param::<Uuid>("enrollment_id")?;
        let service = ctx.get_dependency::<AppModule, dyn EnrollmentService>()?;

        service.remove(&enrollment_id).await?;

        Ok(HttpResponse::Ok())
    }
}
