use std::{
    collections::HashMap,
    env::{temp_dir, var},
    process::Command,
    sync::{Arc, LazyLock},
};

use async_trait::async_trait;
use shaku::{Component, Interface};
use tera::Tera;

use crate::{
    errors::{PrinterError, ServiceError},
    templates::*,
};

static TEMPLATES: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([(
            "document:practice:authorization",
            include_str!("./templates/authorization.typ"),
        )])
    });

#[derive(Component)]
#[shaku(interface = Printer)]
pub struct DocumentPrinter {
    templates: Arc<Tera>,
    context: TemplateContext,
}

pub struct PrintOptions {
    pub doc_id: String,
    pub template: &'static str,
    pub context: RawContext,
}

#[async_trait]
pub trait Printer: Interface {
    fn print(&self, options: PrintOptions) -> Result<String, ServiceError>;
}

impl DocumentPrinter {
    pub async fn new() -> Result<Self, ServiceError> {
        let mut context = TemplateContext::new();

        let interal_ctx = vec![
            (
                "public_url",
                var("PUBLIC_URL")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            ),
            (
                "secretary_email",
                var("SECRETARY_EMAIL")
                    .expect("SECRETARY_EMAIL must be set in the environment"),
            ),
        ];

        context.insert_ctx(interal_ctx);

        Ok(DocumentPrinter {
            templates: TemplateHandler::new(TEMPLATES.clone())?,
            context,
        })
    }
}

impl From<DocumentPrinter> for DocumentPrinterParameters {
    fn from(printer: DocumentPrinter) -> Self {
        DocumentPrinterParameters {
            templates: printer.templates,
            context: printer.context,
        }
    }
}

#[async_trait]
impl Printer for DocumentPrinter {
    fn print(
        &self,
        PrintOptions {
            doc_id,
            template,
            context,
        }: PrintOptions,
    ) -> Result<String, ServiceError> {
        let mut local_context = self.context.clone();
        local_context.insert_ctx(context);

        let template_name = format!("{template}.typ");
        let template = self
            .templates
            .render(&template_name, &local_context.tera_ctx)
            .map_err(|source| ServiceError::Printer {
                source: source.into(),
            })?;

        let temp_file = format!("{}/{template_name}.typ", temp_dir().display());
        let out_file = format!("{}/{doc_id}.pdf", temp_dir().display());

        std::fs::write(&temp_file, template).map_err(|source| {
            ServiceError::Printer {
                source: source.into(),
            }
        })?;

        let output = Command::new("typst")
            .args(["compile", &temp_file, "-o", &out_file])
            .output()
            .map_err(|source| ServiceError::Printer {
                source: source.into(),
            })?;

        if !output.status.success() {
            return Err(PrinterError::PdfGenerationError(
                "Failed to generate PDF".to_string(),
            )
            .into());
        }

        if std::fs::remove_file(&temp_file).is_err() {
            return Err(PrinterError::PdfGenerationError(
                "Failed to remove temporary file".to_string(),
            )
            .into());
        }

        Ok(out_file)
    }
}
