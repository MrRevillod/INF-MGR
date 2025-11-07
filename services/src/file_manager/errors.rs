use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileManagerError {
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid filename encoding: {0}")]
    InvalidFilename(#[from] std::str::Utf8Error),

    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("task join error: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
}
