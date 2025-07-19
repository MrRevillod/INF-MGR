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

        if let Some(evaluation_scores) = input.evaluation_scores {
            inscription.evaluation_scores = evaluation_scores;
        }

        self.repository.update(id, inscription).await
    }
}
