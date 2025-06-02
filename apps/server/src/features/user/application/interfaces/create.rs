// This module defines the CreateUserCase Trait/Interface and its
// corresponding Input DTO CreateUserInput.

use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::{Role, User, UserError};

/// Datos requeridos para crear un nuevo usuario.
/// Este DTO representa la entrada cruda recibida desde
/// la capa de infraestructura (por ejemplo, desde un controller).

pub struct CreateUserInput {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}

/// Caso de uso para crear un nuevo usuario.
/// Esta interfaz define un contrato de aplicación desacoplado,
/// que puede tener múltiples implementaciones
/// (por ejemplo, una real para producción y una mock para tests).

#[async_trait]
pub trait CreateUserCase: Interface {
    async fn execute(&self, input: CreateUserInput) -> Result<User, UserError>;
}

/// Implementación de conversión desde la entrada del caso de uso a la
/// entidad de dominio `User`. Se utiliza `TryFrom` porque la creación
/// de un `User` puede fallar (por ejemplo, al encriptar la contraseña).

impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        let now = chrono::Utc::now();

        User {
            id: input.id,
            name: input.name,
            email: input.email,
            validated: false,
            roles: input
                .roles
                .into_iter()
                .filter_map(|role| Role::try_from(role).ok())
                .collect(),
            password: input.password,
            created_at: now,
            updated_at: now,
        }
    }
}
