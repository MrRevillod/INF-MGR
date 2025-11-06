use std::{env, io::Cursor};

use async_zip::base::read::seek::ZipFileReader;
use futures::io::AllowStdIo;
use uuid::Uuid;

use tokio::fs;

pub async fn extract_zip(
    practice_id: &Uuid,
    zip: Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut zip = ZipFileReader::new(AllowStdIo::new(Cursor::new(zip)))
        .await
        .unwrap();

    let docs_dir =
        env::var("DOCUMENTS_DIR").unwrap_or_else(|_| "./documents".to_string());

    let practice_dir = format!("{docs_dir}/practices/{practice_id}/tex_project");
    fs::create_dir_all(&practice_dir).await?;

    for index in 0..zip.file().entries().len() {
        let filename = zip
            .file()
            .entries()
            .get(index)
            .unwrap()
            .filename()
            .as_str()?
            .to_string();

        println!("Archivo: {}", filename);

        let mut buffer = Vec::new();
        let mut reader = zip.reader_with_entry(index).await?;

        reader.read_to_end_checked(&mut buffer).await?;

        let filename = format!("{practice_dir}/{filename}");

        fs::write(filename, &buffer).await?;
    }

    let main_tex_path = format!("{practice_dir}/main.tex");

    Ok(fs::read_to_string(&main_tex_path).await?)
}
