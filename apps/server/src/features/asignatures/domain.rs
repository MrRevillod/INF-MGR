use async_trait::async_trait;
use shaku::Interface;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Evaluation {
    pub id: Uuid,
    pub name: String,
    pub weight: i32,
}

#[derive(Debug, Clone)]
pub struct Asignature {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub evaluations: Vec<Evaluation>,
    pub status: String,
    pub teacher_id: Uuid,
    pub coordinator_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct AsignatureFilter {
    pub code: Option<String>,
    pub name: Option<String>,
    pub user_id: Option<Uuid>,
}

#[async_trait]
pub trait AsignatureRepository: Interface {
    async fn find_all(&self) -> Result<Vec<Asignature>, AsignatureError>;
    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Asignature>, AsignatureError>;

    async fn find_by_filter(
        &self,
        filter: AsignatureFilter,
    ) -> Result<Vec<Asignature>, AsignatureError>;

    async fn create(&self, input: Asignature)
        -> Result<Asignature, AsignatureError>;

    async fn update(
        &self,
        id: &Uuid,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError>;

    async fn delete(&self, id: &Uuid) -> Result<(), AsignatureError>;
}

#[derive(Debug, Error)]
pub enum AsignatureError {
    #[error("Asignature not found")]
    NotFound,
    #[error("Asignature already exists")]
    AlreadyExists,
    #[error("Asignature Database Error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },
    #[error("Identificador inválido (uuid)")]
    InvalidIdentifier,
    #[error("The user is not a teacher")]
    UserIsNotTeacher,
    #[error("The teacher was not found")]
    TeacherNotFound,
    #[error("User error: {0}")]
    ForeignUserError(String),
    #[error("Unknown error: {0}")]
    UknownError(String),
    #[error("Asignature has inscriptions, cannot delete")]
    HasInscriptions,
}
