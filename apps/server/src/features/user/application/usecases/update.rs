use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::features::user::{
    application::{
        interfaces::{UpdateUserCase, UpdateUserInput},
        services::PasswordHasher,
    },
    domain::{User, UserError, UserRepository},
};

#[derive(Component)]
#[shaku(interface = UpdateUserCase)]
pub struct UpdateUserCaseImpl {
    #[shaku(inject)]
    pub repository: Arc<dyn UserRepository>,
    #[shaku(inject)]
    pub hasher: Arc<dyn PasswordHasher>,
}

#[async_trait]
impl UpdateUserCase for UpdateUserCaseImpl {
    async fn execute(
        &self,
        user_id: &str,
        input: UpdateUserInput,
    ) -> Result<User, UserError> {
        let Some(mut user) = self.repository.find_by_id(user_id).await? else {
            return Err(UserError::NotFound);
        };

        if let Some(e) = input.email {
            user.email = e
        }

        if let Some(p) = input.password {
            user.password = self.hasher.hash(&p)?
        }

        if let Some(role) = input.role {
            user.role = role
        }

        self.repository.update(user).await
    }
}
