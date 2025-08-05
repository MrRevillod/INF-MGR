use std::{collections::HashMap, sync::Arc};
use tera::Tera;

use crate::errors::MailerError;

pub struct TemplateHandler;

impl TemplateHandler {
    pub fn new() -> Result<Arc<Tera>, MailerError> {
        let mut tera = Tera::default();
        let templates = HashMap::from([
            (
                "system:welcome.html",
                include_str!("templates/system/welcome.html"),
            ),
            (
                "practice:creation:supervisor.html",
                include_str!("templates/practice/creation/supervisor.html"),
            ),
            (
                "practice:creation:student.html",
                include_str!("templates/practice/creation/student.html"),
            ),
            (
                "practice:approval:supervisor.html",
                include_str!("templates/practice/approval/supervisor.html"),
            ),
            (
                "practice:approval:coordinator.html",
                include_str!("templates/practice/approval/coordinator.html"),
            ),
            (
                "practice:approval:student.html",
                include_str!("templates/practice/approval/student.html"),
            ),
            (
                "practice:rejection:coordinator.html",
                include_str!("templates/practice/rejection/coordinator.html"),
            ),
            (
                "practice:rejection:student.html",
                include_str!("templates/practice/rejection/student.html"),
            ),
        ]);

        for (name, content) in templates {
            tera.add_raw_template(name, &content)
                .map_err(|source| MailerError::TemplateError { source })?;
        }

        Ok(Arc::new(tera))
    }
}
