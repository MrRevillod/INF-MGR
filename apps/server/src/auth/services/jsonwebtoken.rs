use std::sync::Arc;

use crate::{
    config::{AuthConfig, ConfigService},
    shared::{AppResult, errors::AuthError},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub session_id: String,
    pub role: String,
    pub exp: usize, // Unix timestamp (seconds from epoch)
}

#[derive(Clone)]
pub struct TokenConfig {
    pub secret: String,
    pub expiration: usize,
}

pub enum TokenKind {
    Access,
    Refresh,
}

#[derive(Component)]
#[shaku(interface = TokenService)]
pub struct JsonWebTokenService {
    #[shaku(inject)]
    config: Arc<dyn ConfigService>,
}

pub trait TokenService: Interface {
    fn sign(
        &self,
        token_kind: TokenKind,
        session_id: &str,
        user_id: &str,
        role: &str,
    ) -> AppResult<String>;

    fn verify(&self, token_kind: TokenKind, token: &str) -> AppResult<Claims>;
    fn get_token_config(&self, token_kind: TokenKind) -> TokenConfig;
}

impl TokenService for JsonWebTokenService {
    fn sign(
        &self,
        token_kind: TokenKind,
        session_id: &str,
        user_id: &str,
        role: &str,
    ) -> AppResult<String> {
        let TokenConfig { secret, expiration } = self.get_token_config(token_kind);

        // Convert milliseconds to proper Unix timestamp
        let now = Utc::now();
        let exp_duration = Duration::milliseconds(expiration as i64);
        let exp_timestamp = (now + exp_duration).timestamp() as usize;

        let claims = Claims {
            user_id: user_id.to_string(),
            session_id: session_id.to_string(),
            exp: exp_timestamp,
            role: role.to_string(),
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(AuthError::from)?;

        Ok(token)
    }

    fn verify(&self, token_kind: TokenKind, token: &str) -> AppResult<Claims> {
        let TokenConfig { secret, .. } = self.get_token_config(token_kind);

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(AuthError::from)?;

        Ok(token_data.claims)
    }

    fn get_token_config(&self, token_kind: TokenKind) -> TokenConfig {
        let auth_config =
            self.config.inner().get::<AuthConfig>().unwrap_or_default();

        match token_kind {
            TokenKind::Access => TokenConfig {
                secret: auth_config.access_jwt_secret,
                expiration: auth_config.access_exp_ms,
            },
            TokenKind::Refresh => TokenConfig {
                secret: auth_config.refresh_jwt_secret,
                expiration: auth_config.refresh_exp_ms,
            },
        }
    }
}
