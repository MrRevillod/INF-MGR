use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::asignatures::{
    application::CreateAsignatureCase,
    domain::{Asignature, AsignatureError, AsignatureFilter, AsignatureRepository},
};

#[derive(Component)]
#[shaku(interface = CreateAsignatureCase)]
pub struct CreateAsignatureCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl CreateAsignatureCase for CreateAsignatureCaseImpl {
    async fn execute(
        &self,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError> {
        let filter = AsignatureFilter {
            year: Some(input.year),
            code: Some(input.code.clone()),
            name: Some(input.name.clone()),
        };

        if !self.repository.find_by_filter(filter).await?.is_empty() {
            return Err(AsignatureError::AlreadyExists);
        }

        Ok(self.repository.create(input).await?)
    }
}
