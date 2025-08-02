use serde_json::json;
use sword::web::HttpResponse;
use thiserror::Error;

use crate::shared::services::errors::ServiceError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("User email already exists")]
    EmailAlreadyExists,

    #[error("Invalid email format or domain")]
    InvalidEmail,

    #[error("User RUT already exists")]
    RutAlreadyExists,

    #[error("User database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },

    #[error("Service error: {source}")]
    ServiceError {
        #[from]
        source: ServiceError,
    },

    #[error("Invalid user role: {role}")]
    InvalidRole { role: String },

    #[error("Invalid cursor: {cursor}")]
    InvalidCursor { cursor: String },

    #[error("Inscription error: {0}")]
    ForeignInscriptionError(String),

    #[error("Asignature error: {0}")]
    ForeignAsignatureError(String),
}

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

            UserError::RutAlreadyExists => HttpResponse::BadRequest().data(json!({
                "conflicts": [
                    {
                        "field": "rut",
                        "message": "El RUT proporcionado ya está en uso"
                    }
                ]
            })),

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
