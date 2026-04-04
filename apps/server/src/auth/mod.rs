mod controllers;
mod dtos;
mod entity;
mod middlewares;
mod repositories;
mod services;

pub use middlewares::Authentication;
pub use middlewares::{MinimumRequiredRole, OwnershipValidation};
pub use repositories::{OAuthRepository, SessionRepository};

pub use controllers::AuthController;
pub use dtos::*;
pub use entity::Session;
pub use services::{Claims, JsonWebTokenService, TokenConfig, TokenKind};
pub use services::{OAuthService, SessionService};

use sword::prelude::*;

pub struct AuthModule;

impl Module for AuthModule {
    type Controller = AuthController;

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<OAuthRepository>();
        container.register_component::<SessionRepository>();
        container.register_component::<JsonWebTokenService>();
        container.register_component::<OAuthService>();
        container.register_component::<SessionService>();
    }
}
