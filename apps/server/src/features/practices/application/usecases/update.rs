use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::practices::{
    application::{UpdatePracticeCase, UpdatePracticeInput},
    domain::{Practice, PracticeError, PracticeRepository},
};

#[derive(Component)]
#[shaku(interface = UpdatePracticeCase)]
pub struct UpdatePracticeCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn PracticeRepository>,
}

#[async_trait]
impl UpdatePracticeCase for UpdatePracticeCaseImpl {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdatePracticeInput,
    ) -> Result<Practice, PracticeError> {
        let Some(mut practice) = self.repository.get_by_id(id).await? else {
            return Err(PracticeError::NotFound);
        };

        if let Some(enterprise_name) = input.enterprise_name {
            practice.enterprise_name = enterprise_name;
        }

        if let Some(location) = input.location {
            practice.location = location;
        }

        if let Some(description) = input.description {
            practice.description = description;
        }

        if let Some(supervisor_name) = input.supervisor_name {
            practice.supervisor_name = supervisor_name;
        }

        if let Some(supervisor_email) = input.supervisor_email {
            practice.supervisor_email = supervisor_email;
        }

        if let Some(start_date) = input.start_date {
            practice.start_date = start_date;
        }

        if let Some(end_date) = input.end_date {
            practice.end_date = end_date;
        }

        self.repository.update(practice).await
    }
}
