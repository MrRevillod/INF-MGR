// This module defines the GetUsersCase Trait/Interface and its
// corresponding Input format and return type.

use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::{User, UserError};

// The implementation of the GetUsersCase trait
// is in: /features/user/application/use_cases/get.rs

#[async_trait]
pub trait GetUsersCase: Interface {
    async fn execute(&self) -> Result<Vec<User>, UserError>;
}
