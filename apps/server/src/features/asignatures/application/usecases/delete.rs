use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::asignatures::{
    application::DeleteAsignatureCase,
    domain::{AsignatureError, AsignatureRepository},
};

#[derive(Component)]
#[shaku(interface = DeleteAsignatureCase)]
pub struct DeleteAsignatureCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl DeleteAsignatureCase for DeleteAsignatureCaseImpl {
    async fn execute(&self, id: &Uuid) -> Result<(), AsignatureError> {
        let asignature = self.repository.find_by_id(id).await?;

        let Some(_) = asignature else {
            return Err(AsignatureError::NotFound);
        };

        self.repository.delete(id).await?;

        Ok(())
    }
}
