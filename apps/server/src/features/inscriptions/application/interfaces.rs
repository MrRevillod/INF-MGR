use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::inscriptions::domain::{
    Inscription, InscriptionError, InscriptionFilter, StudentEvaluation,
};

#[async_trait]
pub trait GetInscriptionsCase: Interface {
    async fn execute(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<Inscription>, InscriptionError>;
}

#[async_trait]
pub trait CreateInscriptionCase: Interface {
    async fn execute(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError>;
}

pub struct UpdateInscriptionInput {
    pub practice_id: Option<Uuid>,
    pub evaluation_scores: Option<Vec<StudentEvaluation>>,
    pub status: Option<String>,
}

#[async_trait]
pub trait UpdateInscriptionCase: Interface {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdateInscriptionInput,
    ) -> Result<Inscription, InscriptionError>;
}

#[async_trait]
pub trait DeleteInscriptionCase: Interface {
    async fn execute(&self, id: &Uuid) -> Result<(), InscriptionError>;
}
