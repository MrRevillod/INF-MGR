use crate::shared::AppResult;
use shaku::{Component, Interface};

pub struct Claims {
    pub user_id: String,
    pub session_id: String,
    pub exp: usize,
}

pub struct TokenConfig {
    pub secret: String,
    pub expiration: usize,
}

pub enum TokenKind {
    Access,
    Refresh,
    Other(TokenConfig),
}

#[derive(Component)]
#[shaku(interface = TokenService)]
pub struct JsonWebTokenService {
    access_config: TokenConfig,
    refresh_config: TokenConfig,
}

pub trait TokenService: Interface {
    fn sign(&self, token_kind: TokenKind, user_id: &str) -> AppResult<(String, Claims)>;
    fn verify(&self, token: &str, token_kind: &str) -> AppResult<Claims>;
}

impl JsonWebTokenService {
    pub fn new(access_config: TokenConfig, refresh_config: TokenConfig) -> Self {
        Self {
            access_config,
            refresh_config,
        }
    }
}

impl TokenService for JsonWebTokenService {
    fn sign(&self, _token_kind: TokenKind, _user_id: &str) -> AppResult<(String, Claims)> {
        unimplemented!()
    }

    fn verify(&self, _token: &str, _token_kind: &str) -> AppResult<Claims> {
        unimplemented!()
    }
}

impl From<JsonWebTokenService> for JsonWebTokenServiceParameters {
    fn from(service: JsonWebTokenService) -> Self {
        JsonWebTokenServiceParameters {
            access_config: service.access_config,
            refresh_config: service.refresh_config,
        }
    }
}
