use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Hasher error: {source}")]
    Hasher {
        #[from]
        source: BcryptError,
    },

    #[error("Mailer error: {source}")]
    Mailer {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
