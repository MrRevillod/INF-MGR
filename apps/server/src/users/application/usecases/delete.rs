use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::users::{
    application::interfaces::DeleteUserCase,
    domain::{UserError, UserRepository},
};

#[derive(Component)]
#[shaku(interface = DeleteUserCase)]
pub struct DeleteUserCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl DeleteUserCase for DeleteUserCaseImpl {
    async fn execute(&self, user_id: &str) -> Result<(), UserError> {
        if self.repository.find_by_id(user_id).await?.is_none() {
            return Err(UserError::NotFound)?;
        }

        self.repository.delete(user_id).await
    }
}
