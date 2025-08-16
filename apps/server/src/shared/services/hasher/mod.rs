use crate::shared::services::errors::{HasherError, ServiceError};
use passwords::PasswordGenerator;
use shaku::{Component, Interface};

const PASSWORD_HASH_COST: u32 = 10;
const DEFAULT_PASSWORD_LEN: usize = 12;

pub trait PasswordHasher: Interface {
    fn hash(&self, password: &str) -> Result<String, ServiceError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, ServiceError>;
    fn random_password(&self) -> Result<[String; 2], ServiceError>;
    fn generate_password(&self, len: usize) -> Result<String, ServiceError>;
    fn generate_and_hash(
        &self,
        len: usize,
    ) -> Result<(String, String), ServiceError>;
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

    fn random_password(&self) -> Result<[String; 2], ServiceError> {
        let plain = self.generate_password(DEFAULT_PASSWORD_LEN)?;
        let hashed = self.hash(&plain)?;
        Ok([plain, hashed])
    }

    fn generate_password(&self, len: usize) -> Result<String, ServiceError> {
        let length = std::cmp::max(len, 8);

        let pg = PasswordGenerator {
            length,
            numbers: true,
            symbols: true,
            lowercase_letters: true,
            uppercase_letters: true,
            strict: true,
            ..Default::default()
        };

        let pwd = pg.generate_one().map_err(|e| {
            HasherError::password_generation(format!(
                "password generation failed: {e}",
            ))
        })?;

        Ok(pwd)
    }

    fn generate_and_hash(
        &self,
        len: usize,
    ) -> Result<(String, String), ServiceError> {
        let plain = self.generate_password(len)?;
        let hashed = self.hash(&plain)?;
        Ok((plain, hashed))
    }
}
