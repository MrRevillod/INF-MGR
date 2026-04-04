use serde::{Deserialize, Serialize};

/// Chunk de texto listo para embedding y análisis de plagios.
///
/// Estructura plana que representa cada sección/subsección como una unidad
/// independiente optimizada para vectorización.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChunk {
    pub id: String,                // "introduccion", "objetivos_principales"
    pub title: String,             // "Introducción", "Objetivos principales"
    pub content: String,           // Texto limpio listo para embedding
    pub level: u8,                 // 1=section, 2=subsection, 3=subsubsection
    pub parent_id: Option<String>, // ID del padre si es subsección
    pub root_section: String,      // ID de la sección principal (nivel 1) - para filtrado de plagio
}

/// Documento procesado como lista plana de chunks listos para análisis.
///
/// Elimina la complejidad jerárquica y facilita la transformación directa
/// a embeddings para detección de plagios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTex {
    pub chunks: Vec<TextChunk>,
}
