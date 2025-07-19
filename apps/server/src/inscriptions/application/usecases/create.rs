use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::inscriptions::{
    application::CreateInscriptionCase,
    domain::{
        Inscription, InscriptionError, InscriptionFilter, InscriptionRepository,
    },
};

#[derive(Component)]
#[shaku(interface = CreateInscriptionCase)]
pub struct CreateInscriptionCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn InscriptionRepository>,
}

#[async_trait]
impl CreateInscriptionCase for CreateInscriptionCaseImpl {
    async fn execute(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError> {
        let filter = InscriptionFilter {
            user_id: Some(inscription.user_id),
            asignature_id: Some(inscription.asignature_id),
            status: None,
        };

        if !self.repository.find_all(filter).await?.is_empty() {
            return Err(InscriptionError::InscriptionAlreadyExists);
        }

        self.repository.create(inscription).await
    }
}
