use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::users::{
    application::interfaces::GetUsersCase,
    domain::{GetUsersParams, User, UserError, UserRepository},
};

#[derive(Component)]
#[shaku(interface = GetUsersCase)]
pub struct GetUsersCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl GetUsersCase for GetUsersCaseImpl {
    async fn execute(&self, filter: GetUsersParams) -> Result<Vec<User>, UserError> {
        self.repository.find_all(filter).await
    }
}
