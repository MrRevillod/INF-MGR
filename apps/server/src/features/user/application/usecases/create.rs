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

        // Check if the user already exists by username or email
        // in database, this tasks is done in parallel

        let (username, email) = tokio::try_join!(
            self.repository.find_by_username(&user.username),
            self.repository.find_by_email(&user.email)
        )?;

        // Then check if the user already exists synchronously

        if username.is_some() {
            return Err(UserError::UsernameAlreadyExists);
        }

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
