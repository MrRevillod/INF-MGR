mod context;
mod interface;

pub use context::{MailTo, MailerConfig};
pub use interface::{EmailTransport, LettreTransport};

use std::sync::Arc;

use async_trait::async_trait;
use lettre::{Message, Transport, message::header::ContentType};
use shaku::{Component, Interface};

use crate::{
    errors::{MailerError, ServiceError},
    templates::TemplateContext,
};

#[derive(Component)]
#[shaku(interface = Mailer)]
pub struct MailerService {
    #[shaku(inject)]
    transport: Arc<dyn EmailTransport>,
}

#[async_trait]
pub trait Mailer: Interface {
    fn get_config(&self) -> &MailerConfig;
    async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError>;
}

#[async_trait]
impl Mailer for MailerService {
    async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError> {
        let email_from = self.transport.get_config().smtp_username.clone();
        let email_from_fmt = format!("Prácticas y Tesis <{email_from}>");

        let mut ctx = TemplateContext::new();

        ctx.insert_ctx(mail_to.context);

        let template_name = format!("{}.html", mail_to.template);
        let template = self
            .transport
            .get_templates()
            .render(&template_name, &ctx.tera_ctx)
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
