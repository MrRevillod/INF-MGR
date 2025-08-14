use crate::shared::services::errors::{HasherError, ServiceError};
use shaku::{Component, Interface};

const PASSWORD_HASH_COST: u32 = 10;

pub trait PasswordHasher: Interface {
    fn hash(&self, password: &str) -> Result<String, ServiceError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, ServiceError>;

    fn random_password(&self) -> Result<[String; 2], ServiceError> {
        Ok([
            String::from("random_password"),
            self.hash("random_password")?,
        ])
    }
}

#[derive(Component)]
#[shaku(interface = PasswordHasher)]
pub struct BcryptPasswordHasher;

impl PasswordHasher for BcryptPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, ServiceError> {
        Ok(bcrypt::hash(password, PASSWORD_HASH_COST).map_err(HasherError::hash)?)
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        Ok(bcrypt::verify(password, hash).map_err(HasherError::verify)?)
    }
}
