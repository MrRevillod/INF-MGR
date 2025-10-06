mod controllers;
mod dtos;

mod middlewares {
    mod authentication;
    pub use authentication::Authentication;

    mod role;
    pub use role::{MinimumRequiredRole, OwnershipValidation};
}

pub use middlewares::Authentication;
pub use middlewares::{MinimumRequiredRole, OwnershipValidation};

mod repositories {
    mod auth;
    mod session;

    pub use auth::{AuthRepository, OAuthRepository};
    pub use session::{RedisSessionRepository, SessionRepository};
}

mod services {
    mod cookies;
    pub use cookies::CookieBuilder;

    mod oauth;
    pub use oauth::{GoogleOAuthService, OAuthService};

    mod session;
    pub use session::{SessionService, SessionServiceImpl};

    mod jsonwebtoken;
    pub use jsonwebtoken::{
        Claims, JsonWebTokenService, TokenConfig, TokenKind, TokenService,
    };
}

pub use services::{Claims, TokenConfig, TokenKind, TokenService};

pub use controllers::AuthController;
pub use repositories::{
    AuthRepository, OAuthRepository, RedisSessionRepository, SessionRepository,
};

pub use dtos::*;
pub use services::{
    GoogleOAuthService, JsonWebTokenService, OAuthService, SessionService,
    SessionServiceImpl,
};

mod entity;
pub use entity::Session;
