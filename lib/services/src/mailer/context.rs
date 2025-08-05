use std::collections::HashMap;
use tera::Context;

use serde::Deserialize;
use sword::prelude::config;

#[derive(Debug, Clone, Deserialize)]
#[config(key = "mailer")]
pub struct MailerConfig {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub public_url: String,
    pub secretary_email: String,
}

#[derive(Debug, Clone)]
pub struct MailTo {
    pub subject: &'static str,
    pub email: String,
    pub template: &'static str,
    pub context: MailContext,
}

#[derive(Debug, Clone)]
pub struct MailContext {
    pub data: HashMap<String, String>,
}

impl MailContext {
    pub fn new(config: &MailerConfig) -> Self {
        let mut data = HashMap::new();
        data.insert("color:uct_blue", "#00487C".to_string());
        data.insert("color:uct_yellow", "#F2B705".to_string());
        data.insert("color:uct_white", "#FFFFFF".to_string());
        data.insert("color:uct_black", "#000000".to_string());

        data.insert("public_url", config.public_url.clone());
        data.insert("secretary_email", config.secretary_email.clone());

        Self {
            data: HashMap::new(),
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
