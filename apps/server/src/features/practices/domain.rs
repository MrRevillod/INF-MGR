use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shaku::Interface;
use thiserror::Error;
use uuid::Uuid;

pub struct Practice {
    pub id: Uuid,
    pub enterprise_name: String,
    pub location: String,
    pub description: String,
    pub supervisor_name: String,
    pub supervisor_email: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum PracticeError {
    #[error("Database Practice error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },

    #[error("Practice not found")]
    NotFound,
}

#[async_trait]
pub trait PracticeRepository: Interface {
    async fn create(&self, practice: Practice) -> Result<Practice, PracticeError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Practice>, PracticeError>;
    async fn update(&self, practice: Practice) -> Result<Practice, PracticeError>;
    async fn delete(&self, id: &Uuid) -> Result<(), PracticeError>;
}
