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
            AsignatureError::UnexpectedError(message) => {
                eprintln!("AsignatureError (HTTP 500): {message}");
                HttpResponse::InternalServerError()
            }
            AsignatureError::DatabaseError(message) => {
                eprintln!("AsignatureError Database error (HTTP 500): {message}");
                HttpResponse::InternalServerError()
            }
        }
    }
}
