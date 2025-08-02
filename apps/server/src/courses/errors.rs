use sword::web::HttpResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CourseError {
    #[error("Asignature not found")]
    NotFound,
    #[error("Asignature already exists")]
    AlreadyExists,
    #[error("Asignature Database Error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },
    #[error("Identificador inválido (uuid)")]
    InvalidIdentifier,
    #[error("The user has not the required role")]
    InvalidRequiredRole,
    #[error("The teacher was not found")]
    TeacherNotFound,

    #[error("The coodinator was not found")]
    CoordinatorNotFound,
    #[error("User error: {0}")]
    ForeignUserError(String),
    #[error("Unknown error: {0}")]
    UknownError(String),
    #[error("Asignature has inscriptions, cannot delete")]
    HasInscriptions,

    #[error("Foreign inscription error: {0}")]
    ForeignInscriptionError(String),
}

impl From<CourseError> for HttpResponse {
    fn from(value: CourseError) -> Self {
        match value {
            CourseError::InvalidIdentifier => {
                HttpResponse::BadRequest().message("Identificador inválido")
            }
            CourseError::NotFound => {
                HttpResponse::NotFound().message("Asignatura no encontrada")
            }
            CourseError::AlreadyExists => {
                HttpResponse::Conflict().message("La asignatura ya existe")
            }
            CourseError::Database { source } => {
                eprintln!("CourseError internal error (HTTP 500): {source}");
                HttpResponse::InternalServerError()
            }
            CourseError::TeacherNotFound | CourseError::CoordinatorNotFound => {
                HttpResponse::BadRequest()
                    .message("Profesor o coordinador no encontrado")
            }

            CourseError::InvalidRequiredRole => HttpResponse::BadRequest()
                .message("La asignatura requiere un rol específico"),

            CourseError::ForeignUserError(msg) => {
                eprintln!("CourseError::ForeignUserError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en usuarios")
            }

            CourseError::ForeignInscriptionError(msg) => {
                eprintln!("CourseError::ForeignInscriptionError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en inscripciones")
            }

            CourseError::UknownError(source) => {
                eprintln!("CourseError internal error (HTTP 500): {source}");
                HttpResponse::InternalServerError()
            }
            CourseError::HasInscriptions => HttpResponse::Forbidden()
                .message("La asignatura tiene inscripciones"),
        }
    }
}
