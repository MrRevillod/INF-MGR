use shaku::module;

use crate::{courses, enrollments, imports, practices, shared::database::PostgresDatabase, users};

use crate::shared::services::{
    event_queue::{TokioEventQueue, TokioEventSender},
    hasher::BcryptPasswordHasher,
};

pub struct DependencyContainer {
    pub module: AppModule,
}

impl DependencyContainer {
    pub fn new(postgres_conn: PostgresDatabase, sender: TokioEventSender) -> Self {
        let module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_conn.into())
            .with_component_parameters::<TokioEventSender>(sender.into())
            .build();

        DependencyContainer { module }
    }
}

module! {
    pub AppModule {
        components = [
            PostgresDatabase,
            BcryptPasswordHasher,

            TokioEventSender,

            TokioEventQueue,

            courses::PostgresCourseRepository,
            courses::CourseServiceImpl,

            enrollments::PostgresEnrollmentRepository,
            enrollments::EnrollmentServiceImpl,

            users::PostgresUserRepository,
            users::UserServiceImpl,

            imports::ImportServiceImpl,

            practices::PostgresPracticeRepository,
            practices::PracticeServiceImpl,
        ],
        providers = []
    }
}
