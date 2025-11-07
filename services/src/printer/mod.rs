use crate::ServiceResult;
use crate::config::*;
use crate::types::*;

use crate::{
    errors::{PrinterError, ServiceError},
    templates::{PRINTER_TEMPLATES, TemplateContext},
};

#[derive(Clone)]
pub struct Printer {
    template_ctx: TemplateContext,
    config: ServicesConfig,
}

pub struct PrintOptions {
    pub static_path: String,
    pub template: &'static str,
    pub context: RawContext,
}

impl Printer {
    pub fn new(config: &ServicesConfig) -> ServiceResult<Self> {
        Ok(Self {
            template_ctx: TemplateContext::new(
                PRINTER_TEMPLATES.clone(),
                config.templates.clone(),
            )?,
            config: config.clone(),
        })
    }

    pub async fn print(&self, opts: PrintOptions) -> ServiceResult<String> {
        let template = self.template_ctx.render(opts.template, opts.context)?;
        let template_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/printer/templates");

        let temp_file = template_dir.join(format!("{}.typ", opts.template));

        fs::write(&temp_file, template).await.map_err(|source| {
            ServiceError::Printer {
                source: source.into(),
            }
        })?;

        let documents_dir = self.config.printer.documents_dir.clone();
        let out_file = format!("{documents_dir}/{}", opts.static_path);

        let out_path = Path::new(&out_file);

        if let Some(parent) = out_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).await.map_err(|source| {
                ServiceError::Printer {
                    source: source.into(),
                }
            })?;
        }

        let Some(temp_file) = temp_file.to_str() else {
            let _ = fs::remove_file(&temp_file).await;
            return Err(PrinterError::PdfGenerationError(
                "Failed to generate PDF".to_string(),
            ))?;
        };

        let output = Command::new("typst")
            .args(["compile", temp_file, &out_file])
            .output()
            .await
            .map_err(|source| ServiceError::Printer {
                source: source.into(),
            })?;

        if !output.status.success() {
            return Err(PrinterError::PdfGenerationError(
                "Failed to generate PDF".to_string(),
            ))?;
        }

        let _ = fs::remove_file(&temp_file).await;

        Ok(out_file)
    }

    pub async fn archive(&self, path: String, file: Vec<u8>) -> ServiceResult<()> {
        let documents_dir = &self.config.printer.documents_dir;
        let outh_path_str = format!("{documents_dir}/{path}");
        let out_path = Path::new(&outh_path_str);

        if let Some(parent) = out_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).await.map_err(|source| {
                ServiceError::Printer {
                    source: source.into(),
                }
            })?;
        }

        fs::write(out_path, file)
            .await
            .map_err(|source| ServiceError::Printer {
                source: source.into(),
            })?;

        Ok(())
    }
}
