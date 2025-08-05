use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::shared::{
    entities::Pagination,
    errors::{AppError, Input},
};

use services::{
    hasher::PasswordHasher,
    mailer::{MailContext, MailTo, Mailer},
};

use crate::users::{
    dtos::from_string_vec_roles, CreateUserDto, UpdateUserDto, User, UserFilter,
    UserRepository,
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
    ) -> Result<Pagination<User>, AppError>;

    async fn create(&self, user: CreateUserDto) -> Result<User, AppError>;
    async fn update(&self, id: &Uuid, user: UpdateUserDto)
        -> Result<User, AppError>;
    async fn remove(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_all(
        &self,
        filter: UserFilter,
    ) -> Result<Pagination<User>, AppError> {
        self.users.find_all(filter).await
    }

    async fn create(&self, mut input: CreateUserDto) -> Result<User, AppError> {
        let (user_by_rut, user_by_email) = tokio::try_join!(
            self.users.find_by_rut(&input.rut),
            self.users.find_by_email(&input.email)
        )?;

        if user_by_rut.is_some() {
            return Err(AppError::Conflict(Input {
                field: "rut".to_string(),
                message: "Ya existe un usuario con este RUT".to_string(),
                value: input.rut.clone(),
            }));
        }

        if user_by_email.is_some() {
            return Err(AppError::Conflict(Input {
                field: "email".to_string(),
                message: "Ya existe un usuario con este email".to_string(),
                value: input.email.clone(),
            }));
        }

        let context = MailContext::new(self.mailer.get_config())
            .insert("name", &input.name)
            .insert("email", &input.email)
            .insert("password", &input.password);

        let mail_opts = MailTo {
            subject: "Bienvenido (a) a la plataforma",
            email: input.email.clone(),
            template: "welcome",
            context,
        };

        input.password = self.hasher.hash(&input.password)?;

        self.mailer.send(mail_opts).await?;
        self.users.save(User::try_from(input)?).await
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateUserDto,
    ) -> Result<User, AppError> {
        let Some(mut user) = self.users.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "User",
            });
        };

        if let Some(e) = input.email {
            let email_exists = self.users.find_by_email(&e).await?.is_some();

            if email_exists && user.email != e {
                return Err(AppError::Conflict(Input {
                    field: "email".to_string(),
                    message: "Ya existe un usuario con este email".to_string(),
                    value: e.clone(),
                }));
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

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        if self.users.find_by_id(id).await?.is_none() {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "User",
            });
        }

        self.users.delete(id).await
    }
}
