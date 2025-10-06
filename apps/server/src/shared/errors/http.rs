use super::{AppError, AuthError, NotFoundError, ValidationError};
use serde_json::json;
use sword::web::HttpResponse;

impl From<AppError> for HttpResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NotFound { source } => handle_not_found_error(source),
            AppError::Validation { source } => handle_validation_error(source),
            AppError::Unauthorized { source } => handle_auth_error(source),

            AppError::InternalServerError(ref err) => {
                tracing::error!("Internal error: {}", err);
                HttpResponse::InternalServerError()
            }

            AppError::PostgresDatabase { source } => {
                tracing::error!("Database error: {}", source);
                HttpResponse::InternalServerError()
            }

            AppError::RedisDatabase { source } => {
                tracing::error!("Redis error: {}", source);
                HttpResponse::InternalServerError()
            }

            AppError::Service { source } => {
                tracing::error!("Internal Service error: {}", source);
                HttpResponse::InternalServerError()
            }
        }
    }
}

fn handle_not_found_error(error: NotFoundError) -> HttpResponse {
    let (resource_type, identifier) = match &error {
        NotFoundError::User { id } => ("User", id.to_string()),
        NotFoundError::Course { id } => ("Course", id.to_string()),
        NotFoundError::Enrollment { id } => ("Enrollment", id.to_string()),
        NotFoundError::Practice { id } => ("Practice", id.to_string()),
        NotFoundError::Meeting { id } => ("Meeting", id.to_string()),
        NotFoundError::MeetingRequest { id } => ("MeetingRequest", id.to_string()),
    };

    HttpResponse::NotFound().message(format!(
        "{resource_type} de id '{identifier}' no encontrado"
    ))
}

fn handle_validation_error(error: ValidationError) -> HttpResponse {
    let (field, message) = match &error {
        ValidationError::InvalidRole { value } => {
            ("role", format!("Rol '{value}' inválido."))
        }
        ValidationError::InvalidCourseStatus { value } => {
            ("status", format!("Estado de curso '{value}' inválido."))
        }

        ValidationError::InvalidMeetingStatus(value) => {
            ("status", format!("Estado de reunión '{value}' inválido."))
        }

        ValidationError::NotEnoughAttendees => (
            "attendees",
            "Se requieren al menos 2 asistentes para crear una solicitud de reunión"
                .into(),
        ),

        ValidationError::NotAStudent { .. } => {
            ("studentId", "El usuario no es un estudiante".into())
        }
        ValidationError::NotATeacher { user_id } => (
            "teacherId",
            format!("El usuario {user_id} no es un profesor"),
        ),
        ValidationError::CourseHasEnrollments { .. } => (
            "course",
            "El curso tiene inscripciones asociadas y no puede ser eliminado".into(),
        ),
        ValidationError::NoPracticeAssociated => (
            "practice",
            "La inscripción no tiene una práctica asociada".into(),
        ),

        ValidationError::FinalReportUploadExpired => (
            "finalReport",
            "El período para subir el informe final ha expirado".into(),
        ),

        // Constraint violations
        ValidationError::DuplicateEmail { .. } => {
            ("email", "El email ya está en uso".into())
        }
        ValidationError::DuplicateRut { .. } => {
            ("rut", "El RUT ya está en uso".into())
        }
        ValidationError::DuplicateEnrollment { .. } => (
            "enrollment",
            "El estudiante ya está inscrito en el curso".into(),
        ),
        ValidationError::DuplicateCourse { name, year, .. } => (
            "course",
            format!("El curso '{name}' para el año {year} ya existe"),
        ),

        // Related entity validation
        ValidationError::InvalidStudentId { student_id } => (
            "studentId",
            format!("Identificador de estudiante inválido: {student_id}"),
        ),
        ValidationError::InvalidCourseId { course_id } => (
            "courseId",
            format!("Identificador de curso inválido: {course_id}"),
        ),
        ValidationError::InvalidTeacherId { teacher_id } => (
            "teacherId",
            format!("Identificador de profesor inválido: {teacher_id}"),
        ),

        ValidationError::InvalidDatetime(value) => (
            "startDate",
            format!("Fecha y hora inválida: '{value}'. Use el formato ISO 8601."),
        ),
    };

    HttpResponse::BadRequest()
        .message("Invalid request data")
        .errors(json!({ field: [ { "message": message } ] }))
}

fn handle_auth_error(source: AuthError) -> HttpResponse {
    tracing::warn!("Unauthorized access: {source}");

    let message = match source {
        AuthError::OAuthError(_) => "Error de autenticación OAuth".into(),
        AuthError::UserNotFound(email) => {
            format!("El usuario con email '{email}' no está registrado.")
        }
        AuthError::JsonWebTokenError(err) => {
            tracing::error!("JWT error: {}", err);
            "Error autenticación".into()
        }
        AuthError::SessionExpired => "Sesión expirada".into(),
        AuthError::Other(msg) => msg,
    };

    HttpResponse::Unauthorized()
        .message(message)
        .error("AuthError")
}
