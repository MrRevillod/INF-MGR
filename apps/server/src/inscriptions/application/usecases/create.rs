use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::inscriptions::{
    application::CreateInscriptionCase,
    domain::{Inscription, InscriptionError, InscriptionRepository},
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
        self.repository.create(inscription).await
    }
}
