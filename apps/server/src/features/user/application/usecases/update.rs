use async_trait::async_trait;
use chrono::Utc;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

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
        id: String,
        input: UpdateUserInput,
    ) -> Result<User, UserError> {
        let user_id = Uuid::parse_str(&id).map_err(|_| UserError::InvalidId)?;

        let Some(mut user) = self.repository.find_by_id(user_id).await? else {
            return Err(UserError::NotFound);
        };

        if let Some(u) = input.username {
            user.username = u
        }

        if let Some(e) = input.email {
            user.email = e
        }

        if let Some(p) = input.password {
            user.password = self.hasher.hash(&p)?
        }

        user.updated_at = Utc::now();
        self.repository.update(user).await
    }
}
