use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::shared::{
    entities::Pagination,
    services::{MailContext, MailTo, Mailer, PasswordHasher},
};

use crate::users::{
    dtos::from_string_vec_roles, CreateUserDto, UpdateUserDto, User, UserError,
    UserFilter, UserRepository,
};

#[derive(Component)]
#[shaku(interface = UserService)]
pub struct UserServiceImpl {
    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    hasher: Arc<dyn PasswordHasher>,

    #[shaku(inject)]
    mailer: Arc<dyn Mailer>,
}

#[async_trait]
pub trait UserService: Interface {
    async fn get_all(
        &self,
        filter: UserFilter,
    ) -> Result<Pagination<User>, UserError>;

    async fn create(&self, user: CreateUserDto) -> Result<User, UserError>;

    async fn update(
        &self,
        id: &Uuid,
        user: UpdateUserDto,
    ) -> Result<User, UserError>;

    async fn remove(&self, id: &Uuid) -> Result<(), UserError>;
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_all(
        &self,
        filter: UserFilter,
    ) -> Result<Pagination<User>, UserError> {
        self.users.find_all(filter).await
    }

    async fn create(&self, mut input: CreateUserDto) -> Result<User, UserError> {
        let (user_by_rut, user_by_email) = tokio::try_join!(
            self.users.find_by_rut(&input.rut),
            self.users.find_by_email(&input.email)
        )?;

        if user_by_rut.is_some() {
            return Err(UserError::RutAlreadyExists);
        }

        if user_by_email.is_some() {
            return Err(UserError::EmailAlreadyExists);
        }

        input.password = self.hasher.hash(&input.password)?;

        let mail_opts = MailTo {
            subject: "Bienvenido (a) a la plataforma",
            email: input.email.clone(),
            template: "welcome",
        };

        let public_url = self.mailer.get_config().public_url.clone();

        let context = MailContext::new()
            .insert("name", &input.name)
            .insert("email", &input.email)
            .insert("password", &input.password)
            .insert("public_url", &public_url);

        self.mailer.send(mail_opts, context).await?;

        self.users.save(User::try_from(input)?).await
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateUserDto,
    ) -> Result<User, UserError> {
        let Some(mut user) = self.users.find_by_id(id).await? else {
            return Err(UserError::NotFound);
        };

        if let Some(e) = input.email {
            let email_exists = self.users.find_by_email(&e).await?.is_some();

            if email_exists && user.email != e {
                return Err(UserError::EmailAlreadyExists);
            }

            user.email = e
        }

        if let Some(p) = input.password {
            user.password = self.hasher.hash(&p)?
        }

        if let Some(roles) = input.roles {
            user.roles = from_string_vec_roles(roles)?;
        }

        self.users.save(user).await
    }

    async fn remove(&self, id: &Uuid) -> Result<(), UserError> {
        if self.users.find_by_id(id).await?.is_none() {
            return Err(UserError::NotFound)?;
        }

        self.users.delete(id).await
    }
}
