use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::practices::{
    application::GetPracticeCase,
    domain::{Practice, PracticeError, PracticeRepository},
};

#[derive(Component)]
#[shaku(interface = GetPracticeCase)]
pub struct GetPracticeCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn PracticeRepository>,
}

#[async_trait]
impl GetPracticeCase for GetPracticeCaseImpl {
    async fn execute(&self, id: &Uuid) -> Result<Practice, PracticeError> {
        let Some(practice) = self.repository.get_by_id(id).await? else {
            return Err(PracticeError::NotFound);
        };

        Ok(practice)
    }
}
