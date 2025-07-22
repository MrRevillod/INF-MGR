use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::practices::{
    application::DeletePracticeCase,
    domain::{PracticeError, PracticeRepository},
};

#[derive(Component)]
#[shaku(interface = DeletePracticeCase)]
pub struct DeletePracticeCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn PracticeRepository>,
}

#[async_trait]
impl DeletePracticeCase for DeletePracticeCaseImpl {
    async fn execute(&self, id: &Uuid) -> Result<(), PracticeError> {
        if self.repository.get_by_id(id).await?.is_none() {
            return Err(PracticeError::NotFound);
        }

        self.repository.delete(id).await
    }
}
