use chrono::Utc;
use serde::{Deserialize, Serialize};
use tex_parser::TextChunk;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingChunk {
    pub id: Uuid,
    pub practice_id: Uuid,
    pub root_section: String,
    pub chunk_id: String,
    pub title: String,
    pub level: u8,
    pub content: String,
    pub created_at: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkPayload {
    pub practice_id: Uuid,
    pub root_section: String,
    pub chunk_id: String,
    pub title: String,
    pub level: u8,
    pub created_at: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Embedding {
    pub embedding: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<Embedding>,
}

#[derive(Debug, Clone)]
pub struct SimilarChunk {
    pub chunk: EmbeddingChunk,
    pub similarity_score: f32,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub similar_chunks: Vec<SimilarChunk>,
}

impl From<EmbeddingChunk> for ChunkPayload {
    fn from(chunk: EmbeddingChunk) -> Self {
        ChunkPayload {
            practice_id: chunk.practice_id,
            root_section: chunk.root_section,
            chunk_id: chunk.chunk_id,
            title: chunk.title,
            level: chunk.level,
            created_at: chunk.created_at,
        }
    }
}

impl From<(&Uuid, TextChunk)> for EmbeddingChunk {
    fn from((practice_id, chunk): (&Uuid, TextChunk)) -> Self {
        EmbeddingChunk {
            id: Uuid::new_v4(),
            practice_id: *practice_id,
            root_section: chunk.root_section,
            chunk_id: chunk.id,
            title: chunk.title,
            level: chunk.level,
            content: chunk.content,
            created_at: Utc::now().timestamp() as usize,
        }
    }
}
