use shaku::{Component, Interface};

use crate::shared::services::errors::ServiceError;

const PASSWORD_HASH_COST: u32 = 10;

pub trait PasswordHasher: Interface {
    fn hash(&self, password: &str) -> Result<String, ServiceError>;
}

#[derive(Component)]
#[shaku(interface = PasswordHasher)]
pub struct BcryptPasswordHasher;

impl PasswordHasher for BcryptPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, ServiceError> {
        bcrypt::hash(password, PASSWORD_HASH_COST)
            .map_err(|e| ServiceError::Hasher(e.to_string()))
    }
}
