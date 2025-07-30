use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    asignatures::{
        application::DeleteAsignatureCase,
        domain::{AsignatureError, AsignatureRepository},
    },
    inscriptions::domain::{InscriptionFilter, InscriptionRepository},
};

#[derive(Component)]
#[shaku(interface = DeleteAsignatureCase)]
pub struct DeleteAsignatureCaseImpl {
    #[shaku(inject)]
    asignatures: Arc<dyn AsignatureRepository>,

    #[shaku(inject)]
    inscriptions: Arc<dyn InscriptionRepository>,
}

#[async_trait]
impl DeleteAsignatureCase for DeleteAsignatureCaseImpl {
    async fn execute(&self, id: &Uuid) -> Result<(), AsignatureError> {
        if self.asignatures.find_by_id(id).await?.is_none() {
            return Err(AsignatureError::NotFound);
        };

        let inscriptions_filter = InscriptionFilter {
            asignature_id: Some(*id),
            ..Default::default()
        };

        let inscriptions = self
            .inscriptions
            .find_all(inscriptions_filter)
            .await
            .map_err(|e| AsignatureError::UknownError(e.to_string()))?;

        if !inscriptions.is_empty() {
            return Err(AsignatureError::HasInscriptions);
        }

        self.asignatures.delete(id).await?;

        Ok(())
    }
}
