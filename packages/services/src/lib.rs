pub mod config;
pub mod file_manager;
pub mod mailer;

mod errors;

pub use errors::*;

pub mod templates {
    mod context;
    mod files;

    pub use context::*;
    pub use files::*;
}

pub mod printer;

pub mod types {
    use lettre::{AsyncSmtpTransport, Tokio1Executor};

    pub use futures::future::join_all;
    pub use serde::Deserialize;
    pub use std::sync::Arc;
    pub use std::sync::LazyLock;
    pub use sword::core::config;
    pub use tera::{Context, Tera};
    pub use uuid::Uuid;

    pub type RawContext = Vec<(&'static str, String)>;
    pub type Templates = Vec<(&'static str, &'static str)>;

    pub use std::path::Path;
    pub use tokio::{fs, process::Command};

    pub type SmtpTransport = AsyncSmtpTransport<Tokio1Executor>;

    pub use lettre::{
        AsyncTransport, Message,
        message::{Mailbox, header::ContentType},
        transport::smtp::authentication::Credentials,
    };
}
