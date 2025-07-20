use async_trait::async_trait;
use shaku::Interface;
use thiserror::Error;
use uuid::Uuid;

use crate::{asignatures::domain::AsignatureError, users::domain::UserError};

#[derive(Debug, Clone)]
pub struct Inscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asignature_id: Uuid,
    pub practice_id: Option<Uuid>,
    pub evaluations_scores: Vec<StudentEvaluation>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct StudentEvaluation {
    pub id: Uuid,
    pub score: f64,
}

#[derive(Debug, Error)]
pub enum InscriptionError {
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },

    #[error("Inscription not found")]
    NotFound,

    #[error("Invalid inscription state")]
    InvalidStudentState,

    #[error("Invalid status: {status}")]
    InvalidStatus { status: String },

    #[error("Inscription already exists, cannot create a duplicate")]
    InscriptionAlreadyExists,

    #[error("The selected student does not exist: {id}")]
    StudentNotFound { id: Uuid },

    #[error("The selected user is not a student")]
    InvalidStudentRole,

    #[error("User repository error: {source}")]
    UserError {
        #[from]
        source: UserError,
    },

    #[error("Asignature repository error: {source}")]
    AsignatureError {
        #[from]
        source: AsignatureError,
    },

    #[error("Asignature not found: {id}")]
    AsignatureNotFound { id: Uuid },
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
