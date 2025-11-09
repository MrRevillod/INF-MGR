use rig_fastembed::{Client, EmbeddingModel, FastembedModel};
use sword::core::injectable;

mod models;
pub use models::*;

use qdrant_client::{
    Qdrant,
    qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder},
};

use crate::{EmbeddingServiceError, config::EmbeddingServiceConfig};

#[injectable(provider)]
pub struct EmbeddingService {
    model: EmbeddingModel,
    qdrant_client: Qdrant,
}

impl EmbeddingService {
    pub async fn new(
        config: EmbeddingServiceConfig,
    ) -> Result<Self, EmbeddingServiceError> {
        let client = Client::new();
        let model = client.embedding_model(&FastembedModel::AllMiniLML6V2);

        let qdrant_client = Qdrant::from_url(&config.qdrant_url).build()?;

        if !qdrant_client
            .collection_exists(&config.collection_name)
            .await?
        {
            let collection = CreateCollectionBuilder::new(&config.collection_name)
                .vectors_config(
                    VectorParamsBuilder::new(384, Distance::Cosine).build(),
                )
                .build();

            qdrant_client.create_collection(collection).await?;
        }

        Ok(Self {
            model,
            qdrant_client,
        })
    }
}
