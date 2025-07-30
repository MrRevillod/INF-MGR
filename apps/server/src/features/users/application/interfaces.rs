// This module defines the GetUsersCase Trait/Interface and its
// corresponding Input format and return type.

use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::{
    asignatures::domain::Asignature,
    inscriptions::domain::Inscription,
    users::domain::{FindAllReturnType, GetUsersParams, User, UserError},
};

/// Caso de uso para obtener todos los usuarios.
/// implementación: users/application/use_cases/get.rs
#[async_trait]
pub trait GetUsersCase: Interface {
    async fn get_all(
        &self,
        filter: GetUsersParams,
    ) -> Result<FindAllReturnType, UserError>;

    async fn get_student_inscriptions(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<(Asignature, Inscription)>, UserError>;

    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<User, UserError>;
}

/// Caso de uso para crear un nuevo usuario.
/// implementación: users/application/use_cases/create.rs
#[async_trait]
pub trait CreateUserCase: Interface {
    async fn execute(&self, input: User) -> Result<User, UserError>;
}

// ----------

pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
    pub roles: Option<Vec<String>>,
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
