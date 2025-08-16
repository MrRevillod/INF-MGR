use serde::Deserialize;
use sword::prelude::config;

#[derive(Debug, Clone, Deserialize)]
#[config(key = "mailer")]
pub struct MailerConfig {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[derive(Debug, Clone)]
pub struct MailTo {
    pub subject: &'static str,
    pub email: String,
    pub template: &'static str,
    pub context: Vec<(&'static str, String)>,
}
