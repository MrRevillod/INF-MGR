use crate::tex_parser::*;
use std::collections::HashSet;
use std::sync::LazyLock;

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
    set.insert("Anexos");
    set
});

pub fn validate_tex_structure(content: &str) -> Result<(), String> {
    for cap in SECTION_RE.captures_iter(content) {
        let title = &cap[1];
        if !ALLOWED_SECTIONS.contains(title) {
            return Err(format!(
                "Sección no permitida encontrada: '{}'. Solo se permiten las secciones base de la plantilla.",
                title
            ));
        }
    }
    Ok(())
}
