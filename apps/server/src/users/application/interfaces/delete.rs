// This module defines de DeleteUserCase Trait/Interface and its
// corresponding Input format

use async_trait::async_trait;
use shaku::Interface;

use crate::users::domain::UserError;

// The implementation of the DeleteUserCase trait
// is in: /features/user/application/use_cases/delete.rs

#[async_trait]
pub trait DeleteUserCase: Interface {
    async fn execute(&self, user_id: &str) -> Result<(), UserError>;
}
