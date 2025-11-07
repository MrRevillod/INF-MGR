mod errors;
pub use errors::FileManagerError;

use std::{io::Cursor, path::Path};
use sword::core::injectable;
use zip::ZipArchive;

use tokio::{fs, task};

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
        let documents_dir = &self.services_config.file_manager.documents_dir;
        let outdir = Path::new(documents_dir).join(outdir);

        if !outdir.exists() {
            fs::create_dir_all(&outdir).await?
        }

        task::spawn_blocking(move || {
            let mut archive = ZipArchive::new(Cursor::new(bytes))?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = Path::new(&outdir).join(file.name());

                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        std::fs::create_dir_all(p)?;
                    }
                    std::io::copy(&mut file, &mut std::fs::File::create(&outpath)?)?;
                }
            }

            Ok::<(), FileManagerError>(())
        })
        .await??;

        Ok(())
    }
}
