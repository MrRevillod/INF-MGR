use crate::types::*;

#[derive(Debug, Clone, Deserialize)]
#[config(key = "services-config")]
pub struct ServicesConfig {
    pub mailer: MailerConfig,
    pub templates: TemplateConfig,
    pub printer: PrinterConfig,
    pub zipper: ZipperConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailerConfig {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateConfig {
    pub public_url: String,
    pub career_name: String,
    pub career_manager: String,
    pub secretary_email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrinterConfig {
    pub documents_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZipperConfig {
    pub extract_base_dir: String,
}
