use serde::{Deserialize, Serialize};
use uuid::Uuid;

use rig::Embed;

#[derive(Debug, Embed, Serialize, Deserialize)]
pub struct EmbeddingSection {
    pub id: Uuid,
    pub section_type: String,

    #[embed]
    pub content: String,
}
