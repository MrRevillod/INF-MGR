use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::features::user::{
    application::{
        interfaces::{CreateUserCase, CreateUserInput},
        services::PasswordHasher,
    },
    domain::{User, UserError, UserRepository},
};

#[derive(Component)]
#[shaku(interface = CreateUserCase)]
pub struct CreateUserCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
    #[shaku(inject)]
    hasher: Arc<dyn PasswordHasher>,
}

#[async_trait]
impl CreateUserCase for CreateUserCaseImpl {
    async fn execute(&self, input: CreateUserInput) -> Result<User, UserError> {
        // Convert the input dto format to the domain entity
        let mut user = User::from(input);

        let (id, email) = tokio::try_join!(
            self.repository.find_by_id(&user.id),
            self.repository.find_by_email(&user.email)
        )?;

        // Check if the user already exists by ID in database
        if id.is_some() {
            return Err(UserError::IdAlreadyExists);
        }

        // Check if the user already exists by email in database
        if email.is_some() {
            return Err(UserError::EmailAlreadyExists);
        }

        // check disposable/throwaway email
        if !mailchecker::is_valid(&user.email) {
            return Err(UserError::InvalidEmail);
        }

        user.password = self.hasher.hash(&user.password)?;

        self.repository.create(user).await
    }
}
