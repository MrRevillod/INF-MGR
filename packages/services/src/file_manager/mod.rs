mod errors;
pub use errors::FileManagerError;

use std::{env::temp_dir, path::Path};
use sword::core::injectable;
use tokio::{fs, process::Command};
use uuid::Uuid;

use crate::config::ServicesConfig;

#[injectable]
pub struct FileManager {
    services_config: ServicesConfig,
}

impl FileManager {
    pub async fn read_to_string(
        &self,
        filepath: &Path,
    ) -> Result<String, FileManagerError> {
        let documents_dir = &self.services_config.file_manager.documents_dir;

        let content =
            fs::read_to_string(&Path::new(documents_dir).join(filepath)).await?;

        Ok(content)
    }

    pub async fn unzip(
        &self,
        outdir: &Path,
        bytes: Vec<u8>,
    ) -> Result<(), FileManagerError> {
        let temp_zip = temp_dir().join(format!("temp_{}.zip", Uuid::new_v4()));

        fs::write(&temp_zip, bytes).await?;

        let outdir =
            Path::new(&self.services_config.file_manager.documents_dir).join(outdir);

        if !outdir.exists() {
            fs::create_dir_all(&outdir).await?;
        }

        let output = Command::new("unzip")
            .arg("-q")
            .arg("-o")
            .arg(&temp_zip)
            .arg("-d")
            .arg(&outdir)
            .output()
            .await?;

        if !output.status.success() {
            return Err(FileManagerError::Zip(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let _ = fs::remove_file(&temp_zip).await;

        Ok(())
    }
}
