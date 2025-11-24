use reqwest;
use serde_json::json;
use tracing;

use super::models::{Embedding, EmbeddingResponse};
use crate::{EmbeddingServiceError, config::EmbeddingServiceConfig};

#[derive(Clone)]
pub struct EmbeddingClient {
    client: reqwest::Client,
    config: EmbeddingServiceConfig,
}

impl EmbeddingClient {
    pub fn new(config: EmbeddingServiceConfig) -> Self {
        let client = reqwest::Client::new();
        Self { client, config }
    }

    pub async fn embed_text(
        &self,
        text: &str,
    ) -> Result<Embedding, EmbeddingServiceError> {
        let url = self.config.embeddings_model_url.clone();
        let body = json!({
            "model": self.config.embeddings_model_name,
            "input": text
        });

        tracing::debug!("Sending embedding request to {} with body: {:?}", url, body);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Request failed: {:?}", e);
                EmbeddingServiceError::HttpError { source: e }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!(
                "Embedding request failed with status {}: {}",
                status,
                error_text
            );
            return Err(EmbeddingServiceError::NoEmbeddingError);
        }

        let response_text = response.text().await.map_err(EmbeddingServiceError::from)?;
        let embedding_response: EmbeddingResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                tracing::error!(
                    "Failed to deserialize response: {} from text: {}",
                    e,
                    response_text
                );
                EmbeddingServiceError::SerializationError { source: e }
            })?;

        tracing::debug!(
            "Received embedding response with {} data points",
            embedding_response.data.len()
        );

        let Some(embedding) = embedding_response.data.into_iter().next() else {
            tracing::error!("No embedding data in response");
            return Err(EmbeddingServiceError::NoEmbeddingError);
        };

        Ok(embedding)
    }

    pub async fn embed_chunk(
        &self,
        chunk: super::models::EmbeddingChunk,
    ) -> Result<Embedding, EmbeddingServiceError> {
        self.embed_text(&chunk.content).await
    }
}
