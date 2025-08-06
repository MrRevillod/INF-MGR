use bcrypt::BcryptError;
use lettre::address::AddressError;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::Error as SmtpError;
use tera::Error as TeraError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Hasher error: {source}")]
    Hasher {
        #[from]
        source: HasherError,
    },

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
}

#[derive(Debug, Error)]
pub enum HasherError {
    #[error("Hash error: {source}")]
    Hash { source: BcryptError },

    #[error("Verify error: {source}")]
    Verify { source: BcryptError },
}

impl HasherError {
    pub fn hash(source: BcryptError) -> Self {
        Self::Hash { source }
    }

    pub fn verify(source: BcryptError) -> Self {
        Self::Verify { source }
    }
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

    #[error("Template handling error: {source}")]
    TemplateError {
        #[from]
        source: TeraError,
    },

    #[error("Message building error: {source}")]
    MessageBuild {
        #[from]
        source: LettreError,
    },
}

#[derive(Debug, Error)]
pub enum PrinterError {
    #[error("Template handling error: {source}")]
    TemplateError {
        #[from]
        source: TeraError,
    },

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
