use std::{env::var, fs, path::Path, process::Command};

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

        let template_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/printer/templates");

        let temp_file = template_dir.join(format!("{}.typ", opts.doc_id));

        let out_file = format!(
            "{}/{}.pdf",
            var("DOCUMENTS_DIR").unwrap_or_else(|_| ".".to_string()),
            opts.doc_id
        );

        fs::write(&temp_file, template).map_err(|source| ServiceError::Printer {
            source: source.into(),
        })?;

        let output = Command::new("typst")
            .args(["compile", temp_file.to_str().unwrap(), &out_file])
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

        let _ = std::fs::remove_file(&temp_file);

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
