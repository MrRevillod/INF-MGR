mod client;
mod models;
mod qdrant;

pub use client::*;
pub use models::*;
pub use qdrant::*;

use crate::{EmbeddingServiceError, config::EmbeddingServiceConfig};
use uuid::Uuid;

#[derive(Clone)]
pub struct EmbeddingService {
    embedding_client: EmbeddingClient,
    qdrant_service: QdrantService,
}

impl EmbeddingService {
    pub async fn new(
        config: EmbeddingServiceConfig,
    ) -> Result<Self, EmbeddingServiceError> {
        let embedding_client = EmbeddingClient::new(config.clone());
        let qdrant_service = QdrantService::new(config).await?;

        Ok(Self {
            embedding_client,
            qdrant_service,
        })
    }

    pub async fn embed_text(
        &self,
        text: &str,
    ) -> Result<Embedding, EmbeddingServiceError> {
        self.embedding_client.embed_text(text).await
    }

    pub async fn save_chunk(
        &self,
        chunk: EmbeddingChunk,
    ) -> Result<(), EmbeddingServiceError> {
        let embedding = self.embed_text(&chunk.content.clone()).await?;

        self.qdrant_service.save_chunk(chunk, embedding).await
    }

    pub async fn find_similar_chunks(
        &self,
        content: &str,
        root_section: &str,
        exclude_practice_id: Uuid,
        limit: usize,
        threshold: f32,
    ) -> Result<SearchResult, EmbeddingServiceError> {
        let embedding = self.embed_text(content).await?;
        self.qdrant_service
            .find_similar_chunks(
                embedding,
                root_section,
                exclude_practice_id,
                limit,
                threshold,
            )
            .await
    }
}
