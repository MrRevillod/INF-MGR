use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("OAuthError: {0}")]
    OAuthError(String),

    #[error("The user: {0} isn't registered in the system")]
    UserNotFound(String),

    #[error("Jsonwebtoken error: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error("Session expired")]
    SessionExpired,

    #[error("Other auth error: {0}")]
    Other(String),
}