use crate::{
    config::AuthConfig,
    shared::{AppResult, errors::AuthError},
};

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sword::core::injectable;

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

#[derive(Clone, Copy)]
pub enum TokenKind {
    Access,
    Refresh,
}

#[injectable]
pub struct JsonWebTokenService {
    config: AuthConfig,
}

impl JsonWebTokenService {
    pub fn sign(
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

    pub fn verify(&self, token_kind: TokenKind, token: &str) -> AppResult<Claims> {
        let TokenConfig { secret, .. } = self.get_token_config(token_kind);

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(AuthError::from)?;

        Ok(token_data.claims)
    }

    pub fn get_token_config(&self, token_kind: TokenKind) -> TokenConfig {
        match token_kind {
            TokenKind::Access => TokenConfig {
                secret: self.config.access_jwt_secret.clone(),
                expiration: self.config.access_exp_ms,
            },
            TokenKind::Refresh => TokenConfig {
                secret: self.config.refresh_jwt_secret.clone(),
                expiration: self.config.refresh_exp_ms,
            },
        }
    }
}
