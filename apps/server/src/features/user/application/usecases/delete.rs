use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::features::user::{
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
    async fn execute(&self, id: String) -> Result<(), UserError> {
        let parsed_user_id =
            Uuid::parse_str(&id).map_err(|_| UserError::InvalidId)?;

        if self.repository.find_by_id(parsed_user_id).await?.is_none() {
            return Err(UserError::NotFound)?;
        }

        self.repository.delete(parsed_user_id).await
    }
}
