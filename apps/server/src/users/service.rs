use crate::shared::services::{
    event_queue::{Event, EventQueue},
    hasher::PasswordHasher,
};

use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    shared::{
        entities::{Pagination, DEFAULT_PAGE_SIZE},
        errors::{AppError, Input},
    },
    user_filter,
    users::{
        dtos::from_string_vec_roles, CreateUserDto, UpdateUserDto, User, UserFilter, UserRepository,
    },
};

#[derive(Component)]
#[shaku(interface = UserService)]
pub struct UserServiceImpl {
    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    hasher: Arc<dyn PasswordHasher>,

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,
}

#[async_trait]
pub trait UserService: Interface {
    async fn get_all(&self, filter: UserFilter) -> Result<Pagination<User>, AppError>;

    async fn create(&self, user: CreateUserDto) -> Result<User, AppError>;
    async fn update(&self, id: Uuid, user: UpdateUserDto) -> Result<User, AppError>;
    async fn remove(&self, id: Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_all(&self, filter: UserFilter) -> Result<Pagination<User>, AppError> {
        let results = self.users.find_many(filter.clone()).await?;
        let total = self.users.count(filter.clone()).await?;

        let total_pages = (total as f64 / DEFAULT_PAGE_SIZE as f64).ceil() as u64;

        Ok(Pagination {
            total_pages,
            items: results,
            current_page: filter.page,
            has_previous: filter.page > 1,
            has_next: filter.page < total_pages,
        })
    }

    async fn create(&self, mut input: CreateUserDto) -> Result<User, AppError> {
        let (user_by_rut, user_by_email) = tokio::try_join!(
            self.users.find_one(user_filter! {
                rut: input.rut.clone(),
            }),
            self.users.find_one(user_filter! {
                email: input.email.clone()
            })
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

        let unhashed_password = input.password.clone();

        input.password = self.hasher.hash(&input.password)?;
        let user = self.users.save(User::try_from(input.clone())?).await?;

        let event_data = (user.name.clone(), user.email.clone(), unhashed_password.clone());

        self.event_queue.publish(Event::UserCreated(event_data)).await;

        Ok(user)
    }

    async fn update(&self, id: Uuid, input: UpdateUserDto) -> Result<User, AppError> {
        let Some(mut user) = self.users.find_by_id(&id).await? else {
            return Err(AppError::ResourceNotFound(id));
        };

        if let Some(e) = input.email {
            let email_exists =
                self.users.find_one(user_filter! { email: e.clone() }).await?.is_some();

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

    async fn remove(&self, id: Uuid) -> Result<(), AppError> {
        if self.users.find_by_id(&id).await?.is_none() {
            return Err(AppError::ResourceNotFound(id));
        }

        self.users.delete(&id).await
    }
}
