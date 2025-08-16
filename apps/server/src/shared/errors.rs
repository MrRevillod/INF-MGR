use crate::shared::services::errors::ServiceError;
use serde_json::json;
use sword::web::HttpResponse;
use thiserror::Error;
use uuid::Uuid;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Service error: {source}")]
    Service {
        #[from]
        source: ServiceError,
    },

    #[error("Database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },

    #[error("Not found: {0}")]
    ResourceNotFound(Uuid),

    #[error("Conflict error: {0:?}")]
    Conflict(Input),

    #[error("Invalid input: {0:?}")]
    InvalidInput(Input),

    #[error("Internal server error")]
    InternalServerError(Box<dyn std::error::Error + Send + Sync>),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl From<AppError> for HttpResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::ResourceNotFound(id) => {
                HttpResponse::NotFound().message(format!("Resource not found: {id}"))
            }

            AppError::Conflict(input) => HttpResponse::Conflict().data(json!({
                "field": input.field,
                "value": input.value,
                "message": input.message,
            })),

            AppError::InvalidInput(input) => HttpResponse::BadRequest().data(json!({
                "field": input.field,
                "value": input.value,
                "message": input.message,
            })),

            AppError::InvalidOperation(message) => HttpResponse::BadRequest().message(message),

            _ => {
                tracing::error!("Internal AppError: {error:?}");

                HttpResponse::InternalServerError()
                    .data(json!({ "error": "Error interno del servidor" }))
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Input {
    pub field: String,
    pub message: String,
    pub value: String,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid input in field '{}': {} (value: '{}')",
            self.field, self.message, self.value
        )
    }
}
