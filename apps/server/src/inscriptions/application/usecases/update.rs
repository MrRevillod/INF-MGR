use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::inscriptions::{
    application::{UpdateInscriptionCase, UpdateInscriptionInput},
    domain::{Inscription, InscriptionError, InscriptionRepository},
};

#[derive(Component)]
#[shaku(interface = UpdateInscriptionCase)]
pub struct UpdateInscriptionCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn InscriptionRepository>,
}

#[async_trait]
impl UpdateInscriptionCase for UpdateInscriptionCaseImpl {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdateInscriptionInput,
    ) -> Result<Inscription, InscriptionError> {
        let Some(mut inscription) = self.repository.find_by_id(id).await? else {
            return Err(InscriptionError::NotFound);
        };

        if let Some(evaluations_scores) = input.evaluation_scores {
            inscription.evaluations_scores = evaluations_scores;
        }

        if let Some(status) = input.status {
            inscription.status = status;
        }

        if let Some(practice_id) = input.practice_id {
            inscription.practice_id = Some(practice_id);
        }

        self.repository.update(id, inscription).await
    }
}
