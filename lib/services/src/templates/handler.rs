use std::{collections::HashMap, sync::Arc};
use tera::Tera;

use crate::errors::MailerError;
pub struct TemplateHandler;

impl TemplateHandler {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        templates: HashMap<&'static str, &'static str>,
    ) -> Result<Arc<Tera>, MailerError> {
        let mut tera = Tera::default();

        for (name, content) in templates {
            tera.add_raw_template(name, content)
                .map_err(|source| MailerError::TemplateError { source })?;
        }

        Ok(Arc::new(tera))
    }
}
