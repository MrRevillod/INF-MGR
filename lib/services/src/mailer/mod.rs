mod context;

pub use context::{MailTo, MailerConfig};

use async_trait::async_trait;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

use shaku::{Component, Interface};

use crate::{
    errors::{MailerError, ServiceError},
    templates::{MAILER_TEMPLATES, TemplateConfig, TemplateContext},
};

#[derive(Component)]
#[shaku(interface = Mailer)]
pub struct MailerService {
    transport: SmtpTransport,
    config: MailerConfig,
    template_ctx: TemplateContext,
}

impl MailerService {
    pub fn new(
        config: &MailerConfig,
        template_config: &TemplateConfig,
    ) -> Result<Self, ServiceError> {
        let creds = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );

        let transporter = SmtpTransport::relay(&config.smtp_host)
            .map_err(|source| MailerError::SmtpTransport { source })?
            .credentials(creds)
            .build();

        let templates =
            TemplateContext::new(MAILER_TEMPLATES.clone(), template_config.clone())?;

        Ok(MailerService {
            transport: transporter,
            config: config.clone(),
            template_ctx: templates,
        })
    }
}

#[async_trait]
pub trait Mailer: Interface {
    async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError>;
}

#[async_trait]
impl Mailer for MailerService {
    async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError> {
        let email_from = self.config.smtp_username.clone();
        let email_from_fmt = format!("Prácticas y Tesis <{email_from}>");

        let template_name = format!("{}.html", mail_to.template);
        let template = self.template_ctx.render(&template_name, mail_to.context)?;

        let message = Message::builder()
            .from(email_from_fmt.parse().unwrap())
            .to(mail_to.email.parse().unwrap())
            .subject(mail_to.subject)
            .header(ContentType::TEXT_HTML)
            .body(template)
            .map_err(|source| MailerError::MessageBuild { source })?;

        self.transport
            .send(&message)
            .map_err(|source| MailerError::SmtpTransport { source })?;

        Ok(())
    }
}

impl From<MailerService> for MailerServiceParameters {
    fn from(mailer: MailerService) -> Self {
        MailerServiceParameters {
            transport: mailer.transport,
            config: mailer.config,
            template_ctx: mailer.template_ctx,
        }
    }
}
