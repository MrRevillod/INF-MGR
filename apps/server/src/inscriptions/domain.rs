use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Inscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asignature_id: Uuid,
    pub practice_id: Option<Uuid>,
    pub evaluation_scores: Vec<StudentEvaluation>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct StudentEvaluation {
    pub id: Uuid,
    pub score: f64,
}

pub enum InscriptionError {
    UnexpectedError(String),
    NotFound,
    InvalidStudentState,
}

pub struct InscriptionFilter {
    pub user_id: Option<Uuid>,
    pub asignature_id: Option<Uuid>,
    pub status: Option<String>,
}

#[async_trait]
pub trait InscriptionRepository: Interface {
    async fn find_all(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<Inscription>, InscriptionError>;

    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Inscription>, InscriptionError>;

    async fn create(
        &self,
        incription: Inscription,
    ) -> Result<Inscription, InscriptionError>;

    async fn update(
        &self,
        id: &Uuid,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError>;

    async fn delete(&self, id: &Uuid) -> Result<(), InscriptionError>;
}
