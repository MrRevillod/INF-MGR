use std::sync::{Arc, LazyLock};

use async_trait::async_trait;
use lettre::{message::header::ContentType, Message, Transport};
use regex::Regex;
use shaku::{Component, Interface};
use tera::Context;

use crate::{
    config::MailerConfig,
    shared::{services::errors::ServiceError, smtp::MailerTransport},
};

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

#[derive(Debug, Clone)]
pub struct MailTo {
    pub subject: &'static str,
    pub email: String,
    pub template: &'static str,
}

#[derive(Debug, Clone)]
pub struct MailContext {
    pub data: std::collections::HashMap<String, String>,
}

impl MailContext {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn insert(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), value.to_string());
        self
    }

    pub fn apply_to_tera_context(&self, context: &mut Context) {
        for (key, value) in &self.data {
            context.insert(key, value);
        }
    }
}

#[async_trait]
pub trait Mailer: Interface {
    fn is_valid_email(&self, email: &str) -> bool;
    async fn send(
        &self,
        mail_to: MailTo,
        mail_context: MailContext,
    ) -> Result<(), ServiceError>;
    fn get_config(&self) -> &MailerConfig;
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

    async fn send(
        &self,
        mail_to: MailTo,
        mail_context: MailContext,
    ) -> Result<(), ServiceError> {
        if !self.is_valid_email(&mail_to.email) {
            return Err(ServiceError::Mailer {
                source: "Invalid email format".into(),
            });
        }

        let email_from = self.mailer.get_config().smtp_username.clone();
        let email_from_fmt = format!("Prácticas y Tesis <{email_from}>");

        let mut context = Context::new();

        mail_context.apply_to_tera_context(&mut context);

        let template_name = format!("{}.html", mail_to.template);
        let template = self
            .mailer
            .get_templates()
            .render(&template_name, &context)
            .map_err(|e| ServiceError::Mailer {
                source: Box::new(e),
            })?;

        let message = Message::builder()
            .from(email_from_fmt.parse().unwrap())
            .to(mail_to.email.parse().unwrap())
            .subject(mail_to.subject)
            .header(ContentType::TEXT_HTML)
            .body(template)
            .map_err(|e| ServiceError::Mailer {
                source: Box::new(e),
            })?;

        let transport = self.mailer.get_transtport();

        if transport.send(&message).is_err() {
            return Err(ServiceError::Mailer {
                source: "Failed to send email".into(),
            });
        }

        Ok(())
    }

    fn get_config(&self) -> &MailerConfig {
        self.mailer.get_config()
    }
}

impl Default for MailContext {
    fn default() -> Self {
        Self::new()
    }
}
