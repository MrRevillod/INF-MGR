use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

use crate::{
    asignatures,
    shared::{
        database::PostgresDatabase,
        services::{BcryptPasswordHasher, MailerService},
        smtp::LettreTransport,
    },
    users,
};

#[derive(Clone)]
pub struct DependencyContainer {
    pub module: Arc<AppModule>,
}

impl DependencyContainer {
    pub fn new(
        postgres_conn: PostgresDatabase,
        lettre_transport: LettreTransport,
    ) -> Self {
        let module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_conn.into())
            .with_component_parameters::<LettreTransport>(lettre_transport.into())
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
            LettreTransport,
            BcryptPasswordHasher,
            MailerService,

            asignatures::infrastructure::PostgresAsignatureRepository,
            asignatures::application::GetAsignaturesCaseImpl,
            asignatures::application::CreateAsignatureCaseImpl,
            asignatures::application::UpdateAsignatureCaseImpl,
            asignatures::application::DeleteAsignatureCaseImpl,

            users::infrastructure::PostgresUserRepository,
            users::application::GetUsersCaseImpl,
            users::application::CreateUserCaseImpl,
            users::application::UpdateUserCaseImpl,
            users::application::DeleteUserCaseImpl
        ],
        providers = []
    }
}
