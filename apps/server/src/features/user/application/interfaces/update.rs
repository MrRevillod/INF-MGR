// This module constains the UpdateUserCase Trait/Interface
// its corresponding Input format and return type.

// |----------------------------------------------------------------|
// |                 Input entities between layers                  |
// |----------------------------------------------------------------|
// | User Infrastructure Layer (UpdateUserDto) |     Controller     |
// |----------------------------------------------------------------|
// | User Application Layer (UpdateUserInput)  |      Use Case      |
// |----------------------------------------------------------------|
// |         User Domain Layer (User)          |     Repository     |
// |----------------------------------------------------------------|

use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::{User, UserError};

// This input DTO represents the required data to update an existing user.

pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

// Use case definition for updating an existing user.
// implementation in: /features/user/application/use_cases/update.rs

#[async_trait]
pub trait UpdateUserCase: Interface {
    async fn execute(
        &self,
        id: String,
        input: UpdateUserInput,
    ) -> Result<User, UserError>;
}
