mod context;
pub use context::{MailTo, MailerConfig};

use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};

use crate::{
    errors::{MailerError, ServiceError},
    templates::{MAILER_TEMPLATES, TemplateConfig, TemplateContext},
};

pub struct Mailer {
    transport: SmtpTransport,
    config: MailerConfig,
    template_ctx: TemplateContext,
}

impl Mailer {
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

        Ok(Mailer {
            transport: transporter,
            config: config.clone(),
            template_ctx: templates,
        })
    }

    pub async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError> {
        let email_from = self.config.smtp_username.clone();
        let email_from_fmt = format!("Prácticas y Tesis <{email_from}>");

        let template_name = format!("{}.html", mail_to.template);
        let template = self.template_ctx.render(&template_name, mail_to.context)?;

        let from = email_from_fmt
            .parse::<Mailbox>()
            .map_err(|source| MailerError::Address { source })?;

        let to = mail_to
            .email
            .parse::<Mailbox>()
            .map_err(|source| MailerError::Address { source })?;

        let message = Message::builder()
            .from(from)
            .to(to)
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
