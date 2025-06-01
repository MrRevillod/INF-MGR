// This module defines the CreateUserCase Trait/Interface and its
// corresponding Input DTO CreateUserInput.

// |----------------------------------------------------------------|
// |                 Input entities between layers                  |
// |----------------------------------------------------------------|
// | User Infrastructure Layer (CreateUserDto) |     Controller     |
// |------------------------------------------ |--------------------|
// | User Application Layer (CreateUserInput)  |      Use Case      |
// |------------------------------------------ |--------------------|
// |         User Domain Layer (User)          |     Repository     |
// |----------------------------------------------------------------|

use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::features::user::domain::{User, UserError};

/// Datos requeridos para crear un nuevo usuario.
/// Este DTO representa la entrada cruda recibida desde
/// la capa de infraestructura (por ejemplo, desde un controller).

pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
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
            id: Uuid::new_v4(),
            username: input.username,
            email: input.email,
            validated: false,
            password: input.password,
            created_at: now,
            updated_at: now,
        }
    }
}
