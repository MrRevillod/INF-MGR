use std::sync::{Arc, LazyLock};

use async_trait::async_trait;
use lettre::{message::header::ContentType, Message, Transport};
use regex::Regex;
use shaku::{Component, Interface};
use tera::Context;

use crate::shared::{services::errors::ServiceError, smtp::MailerTransport};

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

#[derive(Debug, Clone)]
pub struct MailTo {
    pub subject: &'static str,
    pub email: String,
    pub template: &'static str,
}

#[async_trait]
pub trait Mailer: Interface {
    fn is_valid_email(&self, email: &str) -> bool;
    async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError>;
}

#[derive(Component)]
#[shaku(interface = Mailer)]
pub struct MailerService {
    #[shaku(inject)]
    mailer: Arc<dyn MailerTransport>,
}

#[async_trait]
impl Mailer for MailerService {
    fn is_valid_email(&self, email: &str) -> bool {
        if !EMAIL_REGEX.is_match(email) {
            return false;
        }

        true
    }

    async fn send(&self, mail_to: MailTo) -> Result<(), ServiceError> {
        if !self.is_valid_email(&mail_to.email) {
            return Err(ServiceError::Mailer("Invalid email address".to_string()));
        }

        let email_from = self.mailer.get_config().smtp_username.clone();
        let email_from_fmt = format!("Prácticas y Tesis <{email_from}>");

        let mut context = Context::new();

        context.insert("subject", &mail_to.subject);
        context.insert("email", &mail_to.email);
        context.insert("public_url", &self.mailer.get_config().public_url);

        let template_name = format!("{}.html", mail_to.template);
        let template = self
            .mailer
            .get_templates()
            .render(&template_name, &context)
            .map_err(|e| {
                ServiceError::Mailer(format!("Failed to render template: {e}"))
            })?;

        let message = Message::builder()
            .from(email_from_fmt.parse().unwrap())
            .to(mail_to.email.parse().unwrap())
            .subject(mail_to.subject)
            .header(ContentType::TEXT_HTML)
            .body(template)
            .map_err(|e| {
                ServiceError::Mailer(format!("Failed to build email: {e}"))
            })?;

        let transport = self.mailer.get_transtport();
        let message_ref = Arc::new(message);

        tokio::spawn(async move {
            if transport.send(message_ref.as_ref()).is_err() {
                #[allow(unused_must_use)] // just retry sending
                transport.send(message_ref.as_ref());
            }
        });

        Ok(())
    }
}
