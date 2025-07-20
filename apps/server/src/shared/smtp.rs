use std::sync::Arc;
use tera::Tera;

use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use shaku::{Component, Interface};

use crate::{config::MailerConfig, shared::services::errors::ServiceError};

pub trait MailerTransport: Interface {
    fn get_transtport(&self) -> Arc<SmtpTransport>;
    fn get_config(&self) -> &MailerConfig;
    fn get_templates(&self) -> Arc<Tera>;
}

#[derive(Component)]
#[shaku(interface = MailerTransport)]
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
            .map_err(|e| ServiceError::Mailer {
                source: Box::new(e),
            })?
            .credentials(creds)
            .build();

        Ok(LettreTransport {
            smtp_transport: Arc::new(transporter),
            config: config.clone(),
            templates: Arc::new(Tera::new(&config.templates).map_err(|e| {
                ServiceError::Mailer {
                    source: Box::new(e),
                }
            })?),
        })
    }
}

impl MailerTransport for LettreTransport {
    fn get_transtport(&self) -> Arc<SmtpTransport> {
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
