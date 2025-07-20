use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::asignatures::{
    application::{UpdateAsignatureCase, UpdateAsignatureInput},
    domain::{Asignature, AsignatureError, AsignatureRepository},
};

#[derive(Component)]
#[shaku(interface = UpdateAsignatureCase)]
pub struct UpdateAsignatureCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl UpdateAsignatureCase for UpdateAsignatureCaseImpl {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdateAsignatureInput,
    ) -> Result<Asignature, AsignatureError> {
        let asignature = self.repository.find_by_id(id).await?;

        let Some(a) = asignature else {
            return Err(AsignatureError::NotFound);
        };

        let updated_asignature = Asignature {
            id: a.id,
            year: input.year.unwrap_or(a.year),
            code: input.code.unwrap_or(a.code),
            name: input.name.unwrap_or(a.name),
            evaluations: input.evaluations.unwrap_or(a.evaluations),
            teacher_id: input.teacher_id.unwrap_or(a.teacher_id),
        };

        let updated = self.repository.update(id, updated_asignature).await?;

        Ok(updated)
    }
}
