use lettre::{
    address::AddressError, error::Error as LettreError,
    transport::smtp::Error as SmtpError,
};

use qdrant_client::QdrantError;
use tera::Error as TeraError;
use thiserror::Error;

use crate::file_manager::FileManagerError;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Mailer error: {source}")]
    Mailer {
        #[from]
        source: MailerError,
    },

    #[error("Printer error: {source}")]
    Printer {
        #[from]
        source: PrinterError,
    },

    #[error("Template handler error: {source}")]
    TemplateHandler {
        #[from]
        source: TeraError,
    },

    #[error("File manager error: {source}")]
    FileManager {
        #[from]
        source: FileManagerError,
    },

    #[error("Embedding service error: {source}")]
    EmbeddingService {
        #[from]
        source: EmbeddingServiceError,
    },
}

#[derive(Debug, Error)]
pub enum MailerError {
    #[error("SMTP transport error: {source}")]
    SmtpTransport {
        #[from]
        source: SmtpError,
    },

    #[error("Email address error: {source}")]
    Address {
        #[from]
        source: AddressError,
    },

    #[error("Message building error: {source}")]
    MessageBuild {
        #[from]
        source: LettreError,
    },
}

#[derive(Debug, Error)]
pub enum PrinterError {
    #[error("Archive creation error: {0}")]
    ArchiveError(String),

    #[error("File write error: {source}")]
    FileWriteError {
        #[from]
        source: std::io::Error,
    },

    #[error("PDF generation error: {0}")]
    PdfGenerationError(String),
}

#[derive(Debug, Error)]
pub enum EmbeddingServiceError {
    #[error("Qdrant client error: {source}")]
    QdrantClientError {
        #[from]
        source: QdrantError,
    },
}
