use shaku::{Component, Interface};

use crate::features::user::domain::UserError;

const PASSWORD_HASH_COST: u32 = 10;

pub trait PasswordHasher: Interface {
    fn hash(&self, password: &str) -> Result<String, UserError>;
}

#[derive(Component)]
#[shaku(interface = PasswordHasher)]
pub struct BcryptPasswordHasher;

impl PasswordHasher for BcryptPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, UserError> {
        bcrypt::hash(password, PASSWORD_HASH_COST)
            .map_err(|_| UserError::UnexpectedError)
    }
}
