use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::users::{
    application::{interfaces::UpdateUserCase, UpdateUserInput},
    domain::{User, UserError, UserRepository},
};

use crate::shared::services::PasswordHasher;

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
        user_id: &Uuid,
        input: UpdateUserInput,
    ) -> Result<User, UserError> {
        let Some(mut user) = self.repository.find_by_id(user_id).await? else {
            return Err(UserError::NotFound);
        };

        if let Some(e) = input.email {
            let email_exists = self.repository.find_by_email(&e).await?.is_some();

            if email_exists && user.email != e {
                return Err(UserError::EmailAlreadyExists);
            }

            user.email = e
        }

        if let Some(p) = input.password {
            user.password = self.hasher.hash(&p)?
        }

        if let Some(roles) = input.roles {
            user.roles = roles
        }

        self.repository.update(user).await
    }
}
