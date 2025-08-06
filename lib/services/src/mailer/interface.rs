use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tera::Tera;

use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};
use shaku::{Component, Interface};

use super::MailerConfig;
use crate::{
    errors::{MailerError, ServiceError},
    templates::TemplateHandler,
};

static TEMPLATES: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                "system:welcome.html",
                include_str!("templates/system/welcome.html"),
            ),
            (
                "practice:creation:supervisor.html",
                include_str!("templates/practice/creation/supervisor.html"),
            ),
            (
                "practice:creation:student.html",
                include_str!("templates/practice/creation/student.html"),
            ),
            (
                "practice:approval:supervisor.html",
                include_str!("templates/practice/approval/supervisor.html"),
            ),
            (
                "practice:approval:coordinator.html",
                include_str!("templates/practice/approval/coordinator.html"),
            ),
            (
                "practice:approval:student.html",
                include_str!("templates/practice/approval/student.html"),
            ),
            (
                "practice:rejection:coordinator.html",
                include_str!("templates/practice/rejection/coordinator.html"),
            ),
            (
                "practice:rejection:student.html",
                include_str!("templates/practice/rejection/student.html"),
            ),
        ])
    });

pub trait EmailTransport: Interface {
    fn get_transport(&self) -> Arc<SmtpTransport>;
    fn get_config(&self) -> &MailerConfig;
    fn get_templates(&self) -> Arc<Tera>;
}

#[derive(Component)]
#[shaku(interface = EmailTransport)]
pub struct LettreTransport {
    pub smtp_transport: Arc<SmtpTransport>,
    pub config: MailerConfig,
    pub templates: Arc<Tera>,
}

impl LettreTransport {
    pub async fn new(config: &MailerConfig) -> Result<Self, ServiceError> {
        let creds = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );

        let transporter = SmtpTransport::relay(&config.smtp_host)
            .map_err(|source| MailerError::SmtpTransport { source })?
            .credentials(creds)
            .build();

        let tera = TemplateHandler::new(TEMPLATES.clone())?;

        Ok(LettreTransport {
            smtp_transport: Arc::new(transporter),
            config: config.clone(),
            templates: tera,
        })
    }
}

impl EmailTransport for LettreTransport {
    fn get_transport(&self) -> Arc<SmtpTransport> {
        self.smtp_transport.clone()
    }

    fn get_config(&self) -> &MailerConfig {
        &self.config
    }

    fn get_templates(&self) -> Arc<Tera> {
        self.templates.clone()
    }
}

impl From<LettreTransport> for LettreTransportParameters {
    fn from(lettre_transport: LettreTransport) -> Self {
        LettreTransportParameters {
            smtp_transport: lettre_transport.smtp_transport,
            config: lettre_transport.config,
            templates: lettre_transport.templates,
        }
    }
}
