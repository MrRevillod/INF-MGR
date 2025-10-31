use super::{AuthError, NotFoundError, ValidationError};
use crate::shared::event_handler::ServiceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Service error: {source}")]
    Service {
        #[from]
        source: ServiceError,
    },

    #[error("Database error: {source}")]
    PostgresDatabase {
        #[from]
        source: sqlx::Error,
    },

    #[error("Redis error: {source}")]
    RedisDatabase {
        #[from]
        source: redis::RedisError,
    },

    #[error("Validation failed: {source}")]
    Validation {
        #[from]
        source: ValidationError,
    },

    #[error("Resource not found: {source}")]
    NotFound {
        #[from]
        source: NotFoundError,
    },

    #[error("Unauthorized: {source}")]
    Unauthorized {
        #[from]
        source: AuthError,
    },

    #[error("Internal server error: {0}")]
    InternalServerError(Box<dyn std::error::Error + Send + Sync>),
}
