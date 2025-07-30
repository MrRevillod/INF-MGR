use sword::web::HttpResponse;

use crate::asignatures::domain::AsignatureError;

impl From<AsignatureError> for HttpResponse {
    fn from(value: AsignatureError) -> Self {
        match value {
            AsignatureError::InvalidIdentifier => {
                HttpResponse::BadRequest().message("Identificador inválido")
            }
            AsignatureError::NotFound => {
                HttpResponse::NotFound().message("Asignatura no encontrada")
            }
            AsignatureError::AlreadyExists => {
                HttpResponse::Conflict().message("La asignatura ya existe")
            }
            AsignatureError::Database { source } => {
                eprintln!("AsignatureError internal error (HTTP 500): {source}");
                HttpResponse::InternalServerError()
            }
            AsignatureError::TeacherNotFound => {
                HttpResponse::BadRequest().message("Profesor no encontrado")
            }
            AsignatureError::UserIsNotTeacher => HttpResponse::BadRequest().message(
                "El profesor seleccionado no se encuentra registrado como docente",
            ),
            AsignatureError::ForeignUserError(msg) => {
                eprintln!("AsignatureError::ForeignUserError: {msg}");
                HttpResponse::InternalServerError()
                    .message("Error inesperado en usuarios")
            }
            AsignatureError::UknownError(source) => {
                eprintln!("AsignatureError internal error (HTTP 500): {source}");
                HttpResponse::InternalServerError()
            }
            AsignatureError::HasInscriptions => HttpResponse::Forbidden()
                .message("La asignatura tiene inscripciones"),
        }
    }
}
