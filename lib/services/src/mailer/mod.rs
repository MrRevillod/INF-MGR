mod context;
mod interface;

pub use context::{MailContext, MailTo, MailerConfig};
pub use interface::{EmailTransport, LettreTransport};

use std::sync::Arc;

use async_trait::async_trait;
use lettre::{Message, Transport, message::header::ContentType};
use shaku::{Component, Interface};
use tera::Context;

use crate::errors::{MailerError, ServiceError};

#[derive(Component)]
#[shaku(interface = Mailer)]
pub struct MailerService {
    #[shaku(inject)]
    transport: Arc<dyn EmailTransport>,
}

#[async_trait]
pub trait Mailer: Interface {
    async fn send(
        &self,
        mail_to: MailTo,
        context: MailContext,
    ) -> Result<(), ServiceError>;
    fn get_config(&self) -> &MailerConfig;
}

#[async_trait]
impl Mailer for MailerService {
    async fn send(
        &self,
        mail_to: MailTo,
        mail_context: MailContext,
    ) -> Result<(), ServiceError> {
        let email_from = self.transport.get_config().smtp_username.clone();
        let email_from_fmt = format!("Prácticas y Tesis <{email_from}>");

        let mut context = Context::new();

        mail_context.apply_to_tera_context(&mut context);

        let template_name = format!("{}.html", mail_to.template);
        let template = self
            .transport
            .get_templates()
            .render(&template_name, &context)
            .map_err(|source| MailerError::TemplateError { source })?;

        let message = Message::builder()
            .from(email_from_fmt.parse().unwrap())
            .to(mail_to.email.parse().unwrap())
            .subject(mail_to.subject)
            .header(ContentType::TEXT_HTML)
            .body(template)
            .map_err(|source| MailerError::MessageBuild { source })?;

        self.transport
            .get_transport()
            .send(&message)
            .map_err(|source| MailerError::SmtpTransport { source })?;

        Ok(())
    }

    fn get_config(&self) -> &MailerConfig {
        self.transport.get_config()
    }
}

impl Default for MailContext {
    fn default() -> Self {
        Self::new()
    }
}
