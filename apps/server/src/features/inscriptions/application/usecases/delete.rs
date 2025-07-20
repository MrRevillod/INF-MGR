use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::inscriptions::{
    application::DeleteInscriptionCase,
    domain::{InscriptionError, InscriptionRepository},
};

#[derive(Component)]
#[shaku(interface = DeleteInscriptionCase)]
pub struct DeleteInscriptionCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn InscriptionRepository>,
}

#[async_trait]
impl DeleteInscriptionCase for DeleteInscriptionCaseImpl {
    async fn execute(&self, id: &Uuid) -> Result<(), InscriptionError> {
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(InscriptionError::NotFound);
        };

        self.repository.delete(id).await
    }
}
