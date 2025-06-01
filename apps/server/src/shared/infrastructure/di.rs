use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

use crate::{
    features::user::{
        application::{
            services::BcryptPasswordHasher,
            usecases::{
                CreateUserCaseImpl, DeleteUserCaseImpl, GetUsersCaseImpl,
                UpdateUserCaseImpl,
            },
        },
        infrastructure::PostgresUserRepository,
    },
    shared::infrastructure::database::PostgresDatabase,
};

pub type Inject<T> = shaku_axum::Inject<AppModule, T>;

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<AppModule>,
}

impl FromRef<AppState> for Arc<AppModule> {
    fn from_ref(app_state: &AppState) -> Arc<AppModule> {
        app_state.module.clone()
    }
}

module! {

    pub AppModule {
        components = [
            PostgresDatabase,
            PostgresUserRepository,

            BcryptPasswordHasher,

            GetUsersCaseImpl,
            CreateUserCaseImpl,
            UpdateUserCaseImpl,
            DeleteUserCaseImpl
        ],
        providers = []
    }
}
