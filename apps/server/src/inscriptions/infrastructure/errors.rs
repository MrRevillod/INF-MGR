use crate::inscriptions::domain::InscriptionError;
use sword::web::HttpResponse;

impl From<InscriptionError> for HttpResponse {
    fn from(error: InscriptionError) -> Self {
        match error {
            InscriptionError::UnexpectedError(message) => {
                eprintln!("|Inscription| - Unexpected error: {message}");
                HttpResponse::InternalServerError().message("Error inesperado")
            }

            InscriptionError::NotFound => {
                HttpResponse::NotFound().message("Asignatura no encontrada")
            }

            InscriptionError::InvalidStudentState => {
                HttpResponse::BadRequest().message("Estado del estudiante inválido")
            }

            InscriptionError::InscriptionAlreadyExists => HttpResponse::Conflict()
                .message(
                    "El estudiante ya se encuentra inscrito en esta asignatura",
                ),
        }
    }
}
