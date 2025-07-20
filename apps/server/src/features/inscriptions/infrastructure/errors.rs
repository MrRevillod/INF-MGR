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

            InscriptionError::InscriptionAlreadyExists => HttpResponse::Conflict()
                .message(
                    "El estudiante ya se encuentra inscrito en esta asignatura",
                ),

            InscriptionError::StudentNotFound { id } => HttpResponse::BadRequest()
                .message("Estudiante no encontrado")
                .data(id),

            InscriptionError::InvalidStudentRole => HttpResponse::Forbidden()
                .message("Estudiante inválido, intente más tarde"),

            InscriptionError::Database { source } => {
                eprintln!("Database error: {}", source);
                HttpResponse::InternalServerError()
            }
            InscriptionError::UserError { source } => {
                eprintln!("User error: {}", source);
                HttpResponse::InternalServerError()
            }
        }
    }
}
