use sword::web::HttpResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InscriptionError {
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },
    #[error("Inscription not found")]
    NotFound,
    #[error("Invalid inscription state")]
    InvalidStudentState,
    #[error("Invalid status: {status}")]
    InvalidStatus { status: String },
    #[error("Inscription already exists, cannot create a duplicate")]
    InscriptionAlreadyExists,
    #[error("The selected student does not exist")]
    StudentNotFound,
    #[error("The selected user is not a student")]
    InvalidStudentRole,
    #[error("User error: {0}")]
    ForeignUserError(String),
    #[error("Course error: {0}")]
    ForeignCourseError(String),
    #[error("Asignature not found")]
    AsignatureNotFound,
}

impl From<InscriptionError> for HttpResponse {
    fn from(error: InscriptionError) -> Self {
        match error {
            InscriptionError::NotFound => {
                HttpResponse::NotFound().message("Asignatura no encontrada")
            }

            InscriptionError::InvalidStudentState => {
                HttpResponse::BadRequest().message("Estado del estudiante inválido")
            }

            InscriptionError::InvalidStatus { status } => HttpResponse::BadRequest()
                .message(format!("Estado inválido: {status}")),

            InscriptionError::InscriptionAlreadyExists => HttpResponse::Conflict()
                .message(
                    "El estudiante ya se encuentra inscrito en esta asignatura",
                ),

            InscriptionError::StudentNotFound => {
                HttpResponse::BadRequest().message("Estudiante no encontrado")
            }

            InscriptionError::InvalidStudentRole => HttpResponse::BadRequest()
                .message("Estudiante inválido, intente más tarde"),

            InscriptionError::Database { source } => {
                eprintln!("Database error: {source}");
                HttpResponse::InternalServerError()
            }

            InscriptionError::AsignatureNotFound => {
                HttpResponse::BadRequest().message("Asignatura no encontrada")
            }

            InscriptionError::ForeignUserError(msg) => {
                eprintln!("InscriptionError::ForeignUserError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en usuarios")
            }
            InscriptionError::ForeignCourseError(msg) => {
                eprintln!("InscriptionError::ForeignAsignatureError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en asignaturas")
            }
        }
    }
}
