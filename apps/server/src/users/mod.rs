mod controllers;
mod dtos;
mod entity;
mod repository;
mod service;

pub use controllers::UsersController;
pub use dtos::{
    CreateUserDto, GetUsersQueryDto, UpdateUserDto, UserResponse, role_validator,
    validate_rut_id,
};

pub use entity::{Administrator, Role, Secretary, Student, Teacher, User};
pub use repository::{UserFilter, UserRepository};
pub use service::UserService;

pub use crate::user_filter;

use sword::prelude::*;

pub struct UsersModule;

impl Module for UsersModule {
    type Controller = UsersController;

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<UserService>();
        container.register_component::<UserRepository>();
    }
}
