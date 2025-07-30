use crate::inscriptions::domain::InscriptionError;
use sword::web::HttpResponse;

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

            InscriptionError::StudentNotFound { id } => HttpResponse::BadRequest()
                .message("Estudiante no encontrado")
                .data(id),

            InscriptionError::InvalidStudentRole => HttpResponse::BadRequest()
                .message("Estudiante inválido, intente más tarde"),

            InscriptionError::Database { source } => {
                eprintln!("Database error: {source}");
                HttpResponse::InternalServerError()
            }

            InscriptionError::AsignatureNotFound { id } => {
                HttpResponse::BadRequest()
                    .message("Asignatura no encontrada")
                    .data(id)
            }

            InscriptionError::ForeignUserError(msg) => {
                eprintln!("InscriptionError::ForeignUserError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en usuarios")
            }
            InscriptionError::ForeignAsignatureError(msg) => {
                eprintln!("InscriptionError::ForeignAsignatureError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en asignaturas")
            }
        }
    }
}
