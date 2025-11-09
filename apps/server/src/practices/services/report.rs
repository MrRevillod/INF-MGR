use std::{collections::HashSet, path::Path, sync::LazyLock};

use services::{ServiceError, file_manager::FileManager};
use sword::core::injectable;
use tex_parser::{LaTexParser, ParsedTex};

use crate::{
    shared::{AppResult, ValidationError},
    types::*,
};

#[injectable]
pub struct PracticeReportService {
    file_manager: Arc<FileManager>,
}

impl PracticeReportService {
    pub async fn save_zipped_project(
        &self,
        practice_id: &Uuid,
        zip_bytes: Vec<u8>,
    ) -> AppResult<()> {
        let path_str = format!("practices/{practice_id}/tex_project");
        let path = Path::new(&path_str);

        self.file_manager
            .unzip(&path, zip_bytes)
            .await
            .map_err(ServiceError::from)?;

        let main_tex_content = self
            .file_manager
            .read_to_string(&path.join("main.tex"))
            .await
            .map_err(ServiceError::from)?;

        let parsed = LaTexParser::parse(&main_tex_content);

        self.validate_tex_structure(&parsed)?;

        dbg!(&parsed);

        Ok(())
    }

    fn validate_tex_structure(&self, parsed_content: &ParsedTex) -> AppResult<()> {
        let sections = &parsed_content
            .chunks
            .iter()
            .filter(|chunk| chunk.level == 1)
            .collect::<Vec<_>>();

        for section in sections {
            if !ALLOWED_SECTIONS.contains(section.title.as_str()) {
                return Err(ValidationError::invalid_tex_structure(
                    section.title.clone(),
                ))?;
            }
        }

        Ok(())
    }
}

pub static ALLOWED_SECTIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("Resumen");
    set.insert("Introducción");
    set.insert("Objetivos y expectativas");
    set.insert("Descripción de la empresa");
    set.insert("Organigrama de la empresa");
    set.insert("Actividades encomendadas");
    set.insert("Tecnologías aplicadas");
    set.insert("Experiencia en el proceso de práctica");
    set.insert("Reflexiones");
    set
});
