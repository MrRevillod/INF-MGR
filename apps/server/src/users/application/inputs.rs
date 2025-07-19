// |----------------------------------------------------------------|
// |                 Input entities between layers                  |
// |----------------------------------------------------------------|
// | User Infrastructure Layer (UpdateUserDto) |     Controller     |
// |----------------------------------------------------------------|
// | User Application Layer ([CASE]UserInput)  |      Use Case      |
// |----------------------------------------------------------------|
// |         User Domain Layer (User)          |     Repository     |
// |----------------------------------------------------------------|

use crate::users::domain::User;
use uuid::Uuid;

/// Datos requeridos para crear un nuevo usuario.
/// Este DTO representa la entrada cruda recibida desde
/// la capa de infraestructura (por ejemplo, desde un controller).
#[derive(Debug, Clone)]
pub struct CreateUserInput {
    pub rut: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

/// Implementación de conversión desde la entrada del caso de uso a la
/// entidad de dominio `User`. Se utiliza `TryFrom` porque la creación
/// de un `User` puede fallar (por ejemplo, al encriptar la contraseña).
impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        User {
            id: Uuid::new_v4(),
            rut: input.rut,
            name: input.name,
            email: input.email,
            role: input.role,
            password: input.password,
        }
    }
}

/// This input DTO represents the required data to update an existing user.
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}
