use sword::web::HttpResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnrollmentError {
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

impl From<EnrollmentError> for HttpResponse {
    fn from(error: EnrollmentError) -> Self {
        match error {
            EnrollmentError::NotFound => {
                HttpResponse::NotFound().message("Asignatura no encontrada")
            }

            EnrollmentError::InvalidStudentState => {
                HttpResponse::BadRequest().message("Estado del estudiante inválido")
            }

            EnrollmentError::InvalidStatus { status } => HttpResponse::BadRequest()
                .message(format!("Estado inválido: {status}")),

            EnrollmentError::InscriptionAlreadyExists => HttpResponse::Conflict()
                .message(
                    "El estudiante ya se encuentra inscrito en esta asignatura",
                ),

            EnrollmentError::StudentNotFound => {
                HttpResponse::BadRequest().message("Estudiante no encontrado")
            }

            EnrollmentError::InvalidStudentRole => HttpResponse::BadRequest()
                .message("Estudiante inválido, intente más tarde"),

            EnrollmentError::Database { source } => {
                eprintln!("Database error: {source}");
                HttpResponse::InternalServerError()
            }

            EnrollmentError::AsignatureNotFound => {
                HttpResponse::BadRequest().message("Asignatura no encontrada")
            }

            EnrollmentError::ForeignUserError(msg) => {
                eprintln!("EnrollmentError::ForeignUserError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en usuarios")
            }
            EnrollmentError::ForeignCourseError(msg) => {
                eprintln!("EnrollmentError::ForeignAsignatureError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en asignaturas")
            }
        }
    }
}
