use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        CreateCollectionBuilder, Distance, PointStruct, UpsertPointsBuilder,
        VectorParamsBuilder,
    },
};

use serde_json::to_value;

use super::models::{ChunkPayload, Embedding, EmbeddingChunk};
use crate::{EmbeddingServiceError, config::EmbeddingServiceConfig};

#[derive(Clone)]
pub struct QdrantService {
    client: Qdrant,
    config: EmbeddingServiceConfig,
}

impl QdrantService {
    pub async fn new(
        config: EmbeddingServiceConfig,
    ) -> Result<Self, EmbeddingServiceError> {
        let client = Qdrant::from_url(&config.qdrant_url).build()?;

        if !client.collection_exists(&config.collection_name).await? {
            let collection = CreateCollectionBuilder::new(&config.collection_name)
                .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine).build())
                .build();

            client.create_collection(collection).await?;
        }

        Ok(Self { client, config })
    }

    pub async fn save_chunk(
        &self,
        chunk: EmbeddingChunk,
        embedding: Embedding,
    ) -> Result<(), EmbeddingServiceError> {
        let vector = embedding
            .embedding
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<f32>>();

        let chunk_id = chunk.id.to_string();
        let payload =
            to_value(ChunkPayload::from(chunk)).map_err(EmbeddingServiceError::from)?;

        let point = PointStruct::new(chunk_id, vector, Payload::try_from(payload)?);

        let query =
            UpsertPointsBuilder::new(&self.config.collection_name, vec![point]).build();

        self.client
            .upsert_points(query)
            .await
            .map_err(EmbeddingServiceError::from)?;

        Ok(())
    }
}
