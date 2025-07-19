use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::{
    shared::services::{MailContext, MailTo, Mailer, PasswordHasher},
    users::{
        application::{inputs::CreateUserInput, interfaces::CreateUserCase},
        domain::{User, UserError, UserRepository},
    },
};

#[derive(Component)]
#[shaku(interface = CreateUserCase)]
pub struct CreateUserCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,

    #[shaku(inject)]
    hasher: Arc<dyn PasswordHasher>,

    #[shaku(inject)]
    mailer: Arc<dyn Mailer>,
}

#[async_trait]
impl CreateUserCase for CreateUserCaseImpl {
    async fn execute(&self, input: CreateUserInput) -> Result<User, UserError> {
        let mut user = User::from(input.clone());

        let (id, email) = tokio::try_join!(
            self.repository.find_by_rut(&user.rut),
            self.repository.find_by_email(&user.email)
        )?;

        if id.is_some() {
            return Err(UserError::IdAlreadyExists);
        }

        if email.is_some() {
            return Err(UserError::EmailAlreadyExists);
        }

        if !mailchecker::is_valid(&user.email) {
            return Err(UserError::InvalidEmail);
        }

        user.password = self.hasher.hash(&user.password)?;

        let mail_opts = MailTo {
            subject: "Bienvenido (a) a la plataforma",
            email: user.email.clone(),
            template: "welcome",
        };

        let public_url = self.mailer.get_config().public_url.clone();

        let context = MailContext::new()
            .insert("name", &user.name)
            .insert("email", &user.email)
            .insert("password", &input.password)
            .insert("public_url", &public_url);

        self.mailer.send(mail_opts, context).await?;

        self.repository.create(user).await
    }
}
