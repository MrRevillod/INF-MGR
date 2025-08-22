use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sword::prelude::config;
use tera::{Context, Tera};

use crate::shared::services::errors::ServiceError;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[config(key = "template-context")]
pub struct TemplateConfig {
    pub public_url: String,
    pub career_name: String,
    pub career_manager: String,
    pub secretary_email: String,
}

pub type RawContext = Vec<(&'static str, String)>;
pub type Templates = Vec<(&'static str, &'static str)>;

#[derive(Debug, Clone)]
pub struct TemplateContext {
    config: TemplateConfig,
    handler: Arc<Tera>,
}

impl TemplateContext {
    pub fn new(templates: Templates, config: TemplateConfig) -> Result<Self, ServiceError> {
        let mut tera_handler = Tera::default();

        for (name, content) in templates {
            tera_handler.add_raw_template(name, content)?;
        }

        Ok(Self {
            config,
            handler: Arc::new(tera_handler),
        })
    }

    fn create_context_with_config(&self) -> Context {
        let mut context = Context::new();
        context.insert("public_url", &self.config.public_url);
        context.insert("career_name", &self.config.career_name);
        context.insert("career_manager", &self.config.career_manager);
        context.insert("secretary_email", &self.config.secretary_email);
        context
    }

    pub fn render(&self, template: &str, ctx: RawContext) -> Result<String, ServiceError> {
        let mut context = self.create_context_with_config();

        for (key, value) in ctx {
            context.insert(key, &value);
        }

        self.handler
            .render(template, &context)
            .map_err(|source| ServiceError::TemplateHandler { source })
    }

    pub fn config(&self) -> &TemplateConfig {
        &self.config
    }
}
