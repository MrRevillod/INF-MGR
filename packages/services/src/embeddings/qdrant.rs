use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        CreateCollectionBuilder, Distance, PointStruct, SearchPointsBuilder,
        UpsertPointsBuilder, VectorParamsBuilder,
    },
};

use serde_json::to_value;
use uuid::Uuid;

use super::models::{
    ChunkPayload, Embedding, EmbeddingChunk, SearchResult, SimilarChunk,
};
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

    pub async fn clear_collection(&self) -> Result<(), EmbeddingServiceError> {
        self.client
            .delete_collection(&self.config.collection_name)
            .await
            .map_err(EmbeddingServiceError::from)?;

        let collection = CreateCollectionBuilder::new(&self.config.collection_name)
            .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine).build())
            .build();

        self.client
            .create_collection(collection)
            .await
            .map_err(EmbeddingServiceError::from)?;

        Ok(())
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
        let payload_data = ChunkPayload::from(chunk.clone());
        let mut payload_map = serde_json::Map::new();

        // Add basic payload fields
        if let Ok(payload_value) = to_value(&payload_data) {
            if let Some(obj) = payload_value.as_object() {
                payload_map.extend(obj.clone());
            }
        }

        // Add content to payload for later retrieval
        payload_map.insert(
            "content".to_string(),
            serde_json::Value::String(chunk.content.clone()),
        );

        let payload = Payload::try_from(serde_json::Value::Object(payload_map))
            .map_err(EmbeddingServiceError::from)?;

        let point = PointStruct::new(chunk_id, vector, payload);

        let query =
            UpsertPointsBuilder::new(&self.config.collection_name, vec![point]).build();

        self.client
            .upsert_points(query)
            .await
            .map_err(EmbeddingServiceError::from)?;

        Ok(())
    }

    pub async fn find_similar_chunks(
        &self,
        embedding: Embedding,
        root_section: &str,
        exclude_practice_id: Uuid,
        limit: usize,
        score_threshold: f32,
    ) -> Result<SearchResult, EmbeddingServiceError> {
        let vector = embedding
            .embedding
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<f32>>();

        // Simple search without complex filtering for now
        let search_request = SearchPointsBuilder::new(
            &self.config.collection_name,
            vector,
            (limit * 2) as u64, // Get more results to filter manually
        )
        .score_threshold(score_threshold)
        .with_payload(true)
        .build();

        let response = self
            .client
            .search_points(search_request)
            .await
            .map_err(EmbeddingServiceError::from)?;

        let similar_chunks = response
            .result
            .into_iter()
            .filter_map(|point| {
                let payload = &point.payload;

                // Basic filtering - skip same practice and match section
                let practice_id_val = payload.get("practice_id")?;
                let root_section_val = payload.get("root_section")?;

                // Extract string values (simplified)
                let practice_id_str = match practice_id_val {
                    qdrant_client::qdrant::Value {
                        kind: Some(qdrant_client::qdrant::value::Kind::StringValue(s)),
                    } => s,
                    _ => return None,
                };

                let point_root_section = match root_section_val {
                    qdrant_client::qdrant::Value {
                        kind: Some(qdrant_client::qdrant::value::Kind::StringValue(s)),
                    } => s,
                    _ => return None,
                };

                // Skip if same practice
                if practice_id_str == &exclude_practice_id.to_string() {
                    return None;
                }

                // CRITICAL: Only compare chunks from the same root_section
                // This prevents comparing "pears with apples" (e.g., Frontend from tecnologías_aplicadas
                // should only match against other tecnologías_aplicadas chunks, not actividades_encomendadas)
                if point_root_section != root_section {
                    return None;
                }

                // Extract other fields
                let chunk_id = payload
                    .get("chunk_id")
                    .and_then(|v| match v.kind.as_ref()? {
                        qdrant_client::qdrant::value::Kind::StringValue(s) => {
                            Some(s.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();

                let title = payload
                    .get("title")
                    .and_then(|v| match v.kind.as_ref()? {
                        qdrant_client::qdrant::value::Kind::StringValue(s) => {
                            Some(s.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();

                // Filter out chunks with empty or invalid titles
                if title.trim().is_empty() || chunk_id.trim().is_empty() {
                    return None;
                }

                // Parse the point ID
                let point_id = match &point.id {
                    Some(id) => match &id.point_id_options {
                        Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(
                            uuid_str,
                        )) => Uuid::parse_str(uuid_str).ok()?,
                        _ => return None,
                    },
                    None => return None,
                };

                let matched_practice_id = Uuid::parse_str(practice_id_str).ok()?;

                // Get content from payload
                let content = match payload.get("content") {
                    Some(qdrant_client::qdrant::Value {
                        kind: Some(qdrant_client::qdrant::value::Kind::StringValue(s)),
                    }) => s.clone(),
                    _ => String::new(),
                };

                // Filter out chunks with empty or too short content
                if content.trim().is_empty() || content.trim().len() < 100 {
                    return None;
                }

                let chunk = EmbeddingChunk {
                    id: point_id,
                    practice_id: matched_practice_id,
                    root_section: point_root_section.clone(),
                    chunk_id,
                    title,
                    level: 1, // Default level
                    content,
                    created_at: 0, // Default timestamp
                };

                Some(SimilarChunk {
                    chunk,
                    similarity_score: point.score,
                })
            })
            .take(limit)
            .collect();

        Ok(SearchResult { similar_chunks })
    }
}
