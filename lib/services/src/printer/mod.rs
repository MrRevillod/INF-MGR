use std::{
    env::{temp_dir, var},
    process::Command,
};

use crate::{
    errors::{PrinterError, ServiceError},
    templates::*,
};

use async_trait::async_trait;
use shaku::{Component, Interface};

#[derive(Component)]
#[shaku(interface = Printer)]
pub struct DocumentPrinter {
    template_ctx: TemplateContext,
}

pub struct PrintOptions {
    pub doc_id: String,
    pub template: &'static str,
    pub context: RawContext,
}

#[async_trait]
pub trait Printer: Interface {
    async fn print(&self, options: PrintOptions) -> Result<String, ServiceError>;
}

impl DocumentPrinter {
    pub fn new(template_config: &TemplateConfig) -> Result<Self, ServiceError> {
        Ok(DocumentPrinter {
            template_ctx: TemplateContext::new(
                PRINTER_TEMPLATES.clone(),
                template_config.clone(),
            )?,
        })
    }
}

#[async_trait]
impl Printer for DocumentPrinter {
    async fn print(&self, opts: PrintOptions) -> Result<String, ServiceError> {
        let template = self.template_ctx.render(opts.template, opts.context)?;

        let temp_file = format!("{}/{}.typ", opts.doc_id, temp_dir().display());
        let out_file = format!(
            "{}/{}.pdf",
            opts.doc_id,
            var("DOCUMENTS_DIR").unwrap_or_else(|_| ".".to_string())
        );

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

impl From<DocumentPrinter> for DocumentPrinterParameters {
    fn from(printer: DocumentPrinter) -> Self {
        DocumentPrinterParameters {
            template_ctx: printer.template_ctx,
        }
    }
}
