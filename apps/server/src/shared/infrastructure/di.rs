use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

use crate::{features::user, shared::infrastructure::database::PostgresDatabase};

pub type Inject<T> = shaku_axum::Inject<AppModule, T>;

#[derive(Clone)]
pub struct DependencyContainer {
    pub module: Arc<AppModule>,
}

impl DependencyContainer {
    pub fn new(postgres_conn: PostgresDatabase) -> Self {
        let module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_conn.into())
            .build();

        DependencyContainer {
            module: Arc::new(module),
        }
    }
}

impl FromRef<DependencyContainer> for Arc<AppModule> {
    fn from_ref(di: &DependencyContainer) -> Arc<AppModule> {
        di.module.clone()
    }
}

module! {

    pub AppModule {
        components = [
            PostgresDatabase,
            user::infrastructure::PostgresUserRepository,
            user::application::services::BcryptPasswordHasher,

            user::application::usecases::GetUsersCaseImpl,
            user::application::usecases::CreateUserCaseImpl,
            user::application::usecases::UpdateUserCaseImpl,
            user::application::usecases::DeleteUserCaseImpl
        ],
        providers = []
    }
}
