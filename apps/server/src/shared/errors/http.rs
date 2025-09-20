use sword::web::HttpResponse;
use serde_json::json;
use super::{AppError, ValidationError, NotFoundError, AuthError};

impl From<AppError> for HttpResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NotFound { source } => handle_not_found_error(source),
            AppError::Validation { source } => handle_validation_error(source),
            AppError::Unauthorized { source } => handle_auth_error(source),
            
            // Infrastructure errors
            AppError::PostgresDatabase { source } => {
                tracing::error!("Database error: {}", source);
                HttpResponse::InternalServerError()
                    .data(json!({ 
                        "error": "DATABASE_ERROR",
                        "message": "An internal database error occurred"
                    }))
            }

            AppError::RedisDatabase { source } => {
                tracing::error!("Redis error: {}", source);
                HttpResponse::InternalServerError()
                    .data(json!({ 
                        "error": "CACHE_ERROR",
                        "message": "An internal cache error occurred"
                    }))
            }

            AppError::Service { source } => {
                tracing::error!("Service error: {}", source);
                HttpResponse::InternalServerError()
                    .data(json!({ 
                        "error": "SERVICE_ERROR",
                        "message": "An internal service error occurred"
                    }))
            }

            AppError::InternalServerError(ref err) => {
                tracing::error!("Internal error: {}", err);
                HttpResponse::InternalServerError()
                    .data(json!({ 
                        "error": "INTERNAL_ERROR",
                        "message": "An internal server error occurred"
                    }))
            }

            AppError::InvalidOperation(ref message) => {
                HttpResponse::BadRequest()
                    .data(json!({ 
                        "error": "INVALID_OPERATION",
                        "message": message 
                    }))
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
    };

    HttpResponse::NotFound().data(json!({
        "error": "RESOURCE_NOT_FOUND",
        "resource_type": resource_type,
        "identifier": identifier,
        "message": error.to_string()
    }))
}

fn handle_validation_error(error: ValidationError) -> HttpResponse {
    let (field, message) = match &error {
        // Parsing/Format errors
        ValidationError::InvalidRole { value } => (
            "role",
            format!("Invalid role '{}'. Expected: student, teacher, secretary, admin", value)
        ),
        ValidationError::InvalidCourseStatus { value } => (
            "status",
            format!("Invalid status '{}'. Expected: active, completed", value)
        ),
        ValidationError::InvalidField { field, message } => (
            field.as_str(),
            message.clone()
        ),

        // Business logic validation
        ValidationError::NotAStudent { user_id } => (
            "studentId",
            format!("User {} is not a student", user_id)
        ),
        ValidationError::NotATeacher { user_id } => (
            "teacherId",
            format!("User {} is not a teacher", user_id)
        ),
        ValidationError::CourseHasEnrollments { course_id } => (
            "courseId",
            format!("Cannot delete course {}: has active enrollments", course_id)
        ),

        // Constraint violations
        ValidationError::DuplicateEmail { email } => (
            "email",
            format!("Email '{}' is already in use", email)
        ),
        ValidationError::DuplicateRut { rut } => (
            "rut",
            format!("RUT '{}' is already in use", rut)
        ),
        ValidationError::DuplicateEnrollment { student_id, course_id } => (
            "enrollment",
            format!("Student {} is already enrolled in course {}", student_id, course_id)
        ),
        ValidationError::DuplicateCourse { name, year, .. } => (
            "course",
            format!("Course '{}' already exists for year {}", name, year)
        ),

        // Related entity validation
        ValidationError::InvalidStudentId { student_id } => (
            "studentId",
            format!("Invalid student ID: {}", student_id)
        ),
        ValidationError::InvalidCourseId { course_id } => (
            "courseId",
            format!("Invalid course ID: {}", course_id)
        ),
        ValidationError::InvalidTeacherId { teacher_id } => (
            "teacherId",
            format!("Invalid teacher ID: {}", teacher_id)
        ),
    };

    HttpResponse::BadRequest().data(json!({
        "error": "VALIDATION_ERROR",
        "field": field,
        "message": message
    }))
}

fn handle_auth_error(source: AuthError) -> HttpResponse {
    tracing::warn!("Unauthorized access: {source}");

    let error_data = match source {
        AuthError::OAuthError(_) => json!({
            "error": "oauth_error",
            "message": "Error en el proceso de autenticación con Google",
        }),
        AuthError::UserNotFound(email) => json!({
            "error": "user_not_found",
            "details": format!("El usuario con email '{}' no está registrado.", email),
        }),
        AuthError::JsonWebTokenError(err) => {
            eprintln!("JWT error: {:?}", err);
            json!({
                "error": "authorization_token_error",
                "details": "No autorizado"
            })
        }
        AuthError::SessionExpired => json!({
            "error": "session_expired",
            "details": "La sesión ha expirado. Por favor, inicie sesión de nuevo."
        }),
        AuthError::Other(msg) => json!({
            "error": "unauthorized",
            "details": msg
        }),
    };

    HttpResponse::Unauthorized().data(error_data)
}