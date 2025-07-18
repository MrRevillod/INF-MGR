// This module defines the GetUsersCase Trait/Interface and its
// corresponding Input format and return type.

use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::users::{
    application::{CreateUserInput, UpdateUserInput},
    domain::{User, UserError},
};

/// Caso de uso para obtener todos los usuarios.
/// implementación: users/application/use_cases/get.rs
#[async_trait]
pub trait GetUsersCase: Interface {
    async fn execute(&self) -> Result<Vec<User>, UserError>;
}

/// Caso de uso para crear un nuevo usuario.
/// implementación: users/application/use_cases/create.rs
#[async_trait]
pub trait CreateUserCase: Interface {
    async fn execute(&self, input: CreateUserInput) -> Result<User, UserError>;
}

/// Caso de uso para actualizar un usuario existente.
/// implementación: users/application/use_cases/update.rs
#[async_trait]
pub trait UpdateUserCase: Interface {
    async fn execute(
        &self,
        user_id: &Uuid,
        input: UpdateUserInput,
    ) -> Result<User, UserError>;
}

/// Caso de uso para eliminar un usuario.
/// implementación: users/application/use_cases/delete.rs
#[async_trait]
pub trait DeleteUserCase: Interface {
    async fn execute(&self, user_id: &Uuid) -> Result<(), UserError>;
}
