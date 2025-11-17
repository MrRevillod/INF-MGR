mod models;
pub use models::*;

use reqwest::Client as ReqwestClient;
use serde_json::to_value;

use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        CreateCollectionBuilder, Distance, PointStruct, UpsertPointsBuilder,
        VectorParamsBuilder,
    },
};

use rig::{
    client::EmbeddingsClient,
    embeddings::{Embedding, EmbeddingModel as RigEmbeddingModel},
    providers::ollama::{Client as OllamaClient, EmbeddingModel, NOMIC_EMBED_TEXT},
};

use crate::{EmbeddingServiceError, config::EmbeddingServiceConfig};

#[derive(Clone)]
pub struct EmbeddingService {
    model: EmbeddingModel<ReqwestClient>,
    qdrant_client: Qdrant,
    config: EmbeddingServiceConfig,
}

impl EmbeddingService {
    pub async fn new(
        config: EmbeddingServiceConfig,
    ) -> Result<Self, EmbeddingServiceError> {
        let client = OllamaClient::builder().base_url(&config.ollama_url).build();
        let model = client.embedding_model(NOMIC_EMBED_TEXT);

        let qdrant_client = Qdrant::from_url(&config.qdrant_url).build()?;

        if !qdrant_client
            .collection_exists(&config.collection_name)
            .await?
        {
            let collection = CreateCollectionBuilder::new(&config.collection_name)
                .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine).build())
                .build();

            qdrant_client.create_collection(collection).await?;
        }

        Ok(Self {
            model,
            qdrant_client,
            config,
        })
    }

    pub async fn embed_chunk(
        &self,
        chunk: EmbeddingChunk,
    ) -> Result<Embedding, EmbeddingServiceError> {
        self.model
            .embed_text(&chunk.content)
            .await
            .map_err(EmbeddingServiceError::from)
    }

    pub async fn save_chunk(
        &self,
        chunk: EmbeddingChunk,
    ) -> Result<(), EmbeddingServiceError> {
        let embedding = self.embed_chunk(chunk.clone()).await?;

        let vector = embedding
            .vec
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<f32>>();

        let chunk_id = chunk.id.to_string();
        let payload =
            to_value(ChunkPayload::from(chunk)).map_err(EmbeddingServiceError::from)?;

        let point = PointStruct::new(chunk_id, vector, Payload::try_from(payload)?);

        let query =
            UpsertPointsBuilder::new(&self.config.collection_name, vec![point]).build();

        self.qdrant_client
            .upsert_points(query)
            .await
            .map_err(EmbeddingServiceError::from)?;

        Ok(())
    }
}
