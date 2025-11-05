use sword::prelude::*;

use crate::{
    auth::{Authentication, MinimumRequiredRole},
    courses::*,
    enrollments::*,
    shared::http::ContextExt,
    types::{Arc, Uuid},
    users::Role,
};

#[controller("/courses")]
#[uses(Authentication)]
#[doc = "Controlador para la gestión de cursos y sus inscripciones\n"]
#[doc = "Requiere autenticación para todas las rutas"]
pub struct CoursesController {
    courses: Arc<CourseService>,
    enrollments: Arc<EnrollmentService>,
}

#[routes]
impl CoursesController {
    #[get("/")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    #[doc = "Obtener todos los cursos registrados en el sistema\n"]
    #[doc = "Se transforma el output sumando al profesor del curso"]
    async fn get_courses(&self) -> HttpResult {
        let courses = self
            .courses
            .get_all(course_filter! {})
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(courses))
    }

    #[get("/{id}")]
    #[uses(MinimumRequiredRole, config = Role::Student)]
    #[doc = "Obtener un curso por su ID"]
    async fn get_course(&self, req: Request) -> HttpResult {
        let course_id = req.param::<Uuid>("id")?;

        let (course, teacher) = self.courses.get_by_id(&course_id).await?;
        let response = CourseResponse::from((course, teacher));

        Ok(HttpResponse::Ok().data(response))
    }

    #[get("/teacher/{teacher_id}")]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Obtener todos los cursos de un profesor específico\n"]
    #[doc = "Se transforma el output sumando al profesor del curso"]
    async fn get_courses_by_teacher(&self, req: Request) -> HttpResult {
        let teacher_id = req.param::<Uuid>("teacher_id")?;
        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required && owner_validation.user_id != teacher_id {
            return Err(HttpResponse::Forbidden()
                .message("No tienes permiso para ver los cursos de este profesor"));
        }

        let courses = self
            .courses
            .get_all(course_filter! { teacher_id })
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(courses))
    }

    #[get("/student/{student_id}")]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Obtener cursos de un estudiante con toda la información\n"]
    async fn get_student_courses(&self, req: Request) -> HttpResult {
        let student_id = req.param::<Uuid>("student_id")?;
        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            self.courses
                .check_is_teacher_course(&student_id, &owner_validation.user_id)
                .await?;
        }

        // get_all() ya devuelve la información completa
        let enrollments = self
            .enrollments
            .get_all(enrollment_filter! { student_id })
            .await?
            .into_iter()
            .map(EnrollmentResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(enrollments))
    }

    #[get("/{id}/students")]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Obtener todos los estudiantes inscritos en un curso específico\n"]
    #[doc = "Se transforma el output sumando la información del estudiante y su práctica (si aplica)\n"]
    #[doc = "Requiere que el usuario autenticado sea el profesor o superiores"]
    async fn get_course_enrollments(&self, req: Request) -> HttpResult {
        let course_id = req.param::<Uuid>("id")?;
        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            self.courses
                .check_is_teacher_course(&course_id, &owner_validation.user_id)
                .await?;
        }

        let enrollments = self
            .enrollments
            .get_all(enrollment_filter! { course_id })
            .await?
            .into_iter()
            .map(EnrollmentResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(enrollments))
    }

    #[post("/")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    #[doc = "Crear un nuevo curso en el sistema"]
    async fn create_course(&self, req: Request) -> HttpResult {
        let input = req.body_validator::<CreateCourseDto>()?;
        let asignature = self.courses.create(input).await?;

        Ok(HttpResponse::Created().data(asignature))
    }

    #[post("/enroll")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    #[doc = "Inscribir un estudiante en un curso"]
    async fn create_enrollment(&self, req: Request) -> HttpResult {
        let input = req.body_validator::<CreateEnrollmentDto>()?;
        let enrollment = self.enrollments.create(input).await?;

        Ok(HttpResponse::Created().data(enrollment))
    }

    #[patch("/{id}")]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Actualizar la información de un curso"]
    #[doc = "Requiere que el usuario autenticado sea el profesor del curso o superiores"]
    async fn update_course(&self, req: Request) -> HttpResult {
        let asignature_id = req.param::<Uuid>("id")?;
        let input = req.body_validator::<UpdateCourseDto>()?;
        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            self.courses
                .check_is_teacher_course(&asignature_id, &owner_validation.user_id)
                .await?;
        }

        let updated_asignature = self.courses.update(&asignature_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_asignature))
    }

    #[patch("/enrollments/{enrollment_id}")]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Actualizar la información de una inscripción"]
    async fn update_enrollment(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("enrollment_id")?;
        let input = req.body_validator::<UpdateEnrollmentDto>()?;
        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            let (enrollment, _, _, _) =
                self.enrollments.get_by_id(&enrollment_id).await?;

            let (_, teacher) = self.courses.get_by_id(&enrollment.course_id).await?;

            if teacher.id != owner_validation.user_id {
                return Err(HttpResponse::Forbidden()
                    .message("No tienes permiso para modificar esta inscripción"));
            }
        }

        let updated_enrollment =
            self.enrollments.update(&enrollment_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_enrollment))
    }

    #[delete("/{id}")]
    #[uses(MinimumRequiredRole, config = Role::Administrator)]
    #[doc = "Eliminar un curso del sistema (solo si no tiene estudiantes inscritos)"]
    async fn remove(&self, req: Request) -> HttpResult {
        let course_id = req.param::<Uuid>("id")?;
        self.courses.remove(&course_id).await?;

        Ok(HttpResponse::Ok())
    }

    #[delete("/enrollments/{enrollment_id}")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    #[doc = "Eliminar una inscripción de un estudiante en un curso"]
    async fn remove_enrollment(&self, req: Request) -> HttpResult {
        let enrollment_id = req.param::<Uuid>("enrollment_id")?;
        self.enrollments.remove(&enrollment_id).await?;

        Ok(HttpResponse::Ok())
    }
}
