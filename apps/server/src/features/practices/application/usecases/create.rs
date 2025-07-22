use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::practices::{
    application::CreatePracticeCase,
    domain::{Practice, PracticeError, PracticeRepository},
};

#[derive(Component)]
#[shaku(interface = CreatePracticeCase)]
pub struct CreatePracticeCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn PracticeRepository>,
}

#[async_trait]
impl CreatePracticeCase for CreatePracticeCaseImpl {
    async fn execute(&self, input: Practice) -> Result<Practice, PracticeError> {
        self.repository.create(input).await
    }
}
