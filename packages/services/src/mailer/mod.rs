use crate::errors::{MailerError, ServiceError};
use crate::templates::{MAILER_TEMPLATES, TemplateContext};
use crate::types::*;
use crate::{ServiceResult, config::*};

#[derive(Debug, Clone)]
pub struct MailTo {
    pub subject: String,
    pub email: String,
    pub template: &'static str,
    pub context: Vec<(&'static str, String)>,
}

#[derive(Clone)]
pub struct Mailer {
    transport: SmtpTransport,
    config: MailerConfig,
    template_ctx: TemplateContext,
}

impl Mailer {
    pub fn new(config: &ServicesConfig) -> Result<Self, ServiceError> {
        let mailer_config = &config.mailer;
        let template_config = &config.templates;

        let credentials = Credentials::new(
            mailer_config.smtp_username.clone(),
            mailer_config.smtp_password.clone(),
        );

        let transporter = SmtpTransport::relay(&mailer_config.smtp_host)
            .map_err(|source| MailerError::SmtpTransport { source })?
            .credentials(credentials)
            .build();

        let templates =
            TemplateContext::new(MAILER_TEMPLATES.clone(), template_config.clone())?;

        Ok(Self {
            transport: transporter,
            config: mailer_config.clone(),
            template_ctx: templates,
        })
    }

    pub async fn send(&self, mail_to: MailTo) -> ServiceResult<()> {
        let email_from_fmt =
            format!("Prácticas y Tesis <{}>", self.config.smtp_username);

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
            .send(message)
            .await
            .map_err(|source| MailerError::SmtpTransport { source })?;

        Ok(())
    }

    pub async fn send_many(&self, mails: Vec<MailTo>) -> ServiceResult<()> {
        let results = join_all(mails.into_iter().map(|mail| self.send(mail))).await;

        for result in results {
            result?;
        }

        Ok(())
    }

    pub const fn context(&self) -> &TemplateContext {
        &self.template_ctx
    }
}
