// This file implements the conversion from the `UserError` enum
// to the `HttpResponse` type.

// This is necessary bc the `UserError` enum is used in the internal
// application and domain layers

// |--------------------------------------------|----------------|
// |  User Infrastructure Layer (HttpResponse)  |   Controller   |
// |--------------------------------------------|----------------|
// |     User Application Layer (UserError)     |    Use Case    |
// |--------------------------------------------|----------------|
// |       User Domain Layer (UserError)        |   Repository   |
// |--------------------------------------------|----------------|

use serde_json::json;
use sword::web::HttpResponse;

use crate::users::domain::UserError;

// Each variant of the `UserError` enum corresponds to a specific error
// that can occur in the user management process.

impl From<UserError> for HttpResponse {
    fn from(value: UserError) -> Self {
        match value {
            UserError::EmailAlreadyExists => HttpResponse::Conflict().data(json!({
                "conflicts": [
                    {
                        "field": "email",
                        "message": "Este correo electrónico ya está en uso"
                    }
                ]
            })),

            UserError::NotFound => HttpResponse::NotFound().data(json!({
                "message": "Usuario no encontrado",
            })),

            UserError::Database { source } => {
                eprintln!("| USER ERROR | - Database internal error: {source}");

                HttpResponse::InternalServerError().data(json!({
                    "message": "Error inesperado",
                }))
            }

            UserError::InvalidEmail => HttpResponse::BadRequest().data(json!({
                "field": "email",
                "message": "El correo electrónico proporcionado no es válido",
            })),

            UserError::RutAlreadyExists { rut } => {
                HttpResponse::BadRequest().data(json!({
                    "conflicts": [
                        {
                            "field": "rut",
                            "value": rut,
                            "message": "El RUT proporcionado ya está en uso"
                        }
                    ]
                }))
            }

            UserError::InvalidRole { role } => {
                HttpResponse::BadRequest().data(json!({
                    "field": "roles",
                    "value": role,
                    "message": "El rol proporcionado no es válido",
                }))
            }

            UserError::ServiceError { source } => {
                eprintln!("| USER ERROR | - Service error: {source}");

                HttpResponse::InternalServerError().data(json!({
                    "message": "Error inesperado",
                }))
            }

            UserError::InvalidCursor { cursor } => {
                HttpResponse::BadRequest().data(json!({
                    "field": "cursor",
                    "value": cursor,
                    "message": "El cursor proporcionado no es válido",
                }))
            }

            UserError::ForeignInscriptionError(msg) => {
                eprintln!("UserError::ForeignInscriptionError: {msg}");
                HttpResponse::InternalServerError().data(json!({
                    "message": "Error inesperado en inscripciones",
                }))
            }
            UserError::ForeignAsignatureError(msg) => {
                eprintln!("UserError::ForeignAsignatureError: {msg}");
                HttpResponse::InternalServerError().data(json!({
                    "message": "Error inesperado en asignaturas",
                }))
            }
        }
    }
}
