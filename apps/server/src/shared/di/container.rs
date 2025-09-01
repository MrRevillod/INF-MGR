use shaku::module;

use crate::shared::database::PostgresDatabase;
use crate::shared::di::builder::DependencyContainerBuilder;
use crate::shared::oauth::GoogleOAuthClient;
use crate::shared::redis::RedisDatabase;
use crate::shared::services::event_queue::*;

use crate::{auth, courses, enrollments, imports, practices, users};

pub struct DependencyContainer {
    pub module: AppModule,
}

impl DependencyContainer {
    pub fn builder() -> DependencyContainerBuilder {
        DependencyContainerBuilder::new()
    }

    pub fn new(
        postgres_db: PostgresDatabase,
        event_sender: TokioEventSender,
        oauth_client: GoogleOAuthClient,
        redis_db: RedisDatabase,
    ) -> Self {
        let module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_db.into())
            .with_component_parameters::<TokioEventSender>(event_sender.into())
            .with_component_parameters::<GoogleOAuthClient>(oauth_client.into())
            .with_component_parameters::<RedisDatabase>(redis_db.into())
            .build();

        DependencyContainer { module }
    }
}

module! {
    pub AppModule {
        components = [
            RedisDatabase,
            PostgresDatabase,
            GoogleOAuthClient,

            TokioEventSender,
            TokioEventQueue,

            auth::AuthRepositoryImpl,
            auth::GoogleOAuthService,

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
