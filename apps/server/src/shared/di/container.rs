use shaku::module;

use crate::auth::JsonWebTokenService;
use crate::shared::database::PostgresDatabase;
use crate::shared::di::builder::DependencyContainerBuilder;
use crate::shared::oauth::GoogleOAuthClient;
use crate::shared::redis::RedisDatabase;
use crate::shared::services::event_queue::*;

use crate::shared::services::mailer::Mailer;
use crate::shared::services::printer::Printer;
use crate::{auth, courses, enrollments, imports, practices, users};

pub type InitialComponents = (
    PostgresDatabase,
    Mailer,
    Printer,
    GoogleOAuthClient,
    RedisDatabase,
    JsonWebTokenService,
);

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
        jwt_service: JsonWebTokenService,
    ) -> Self {
        let module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_db.into())
            .with_component_parameters::<TokioEventSender>(event_sender.into())
            .with_component_parameters::<GoogleOAuthClient>(oauth_client.into())
            .with_component_parameters::<RedisDatabase>(redis_db.into())
            .with_component_parameters::<JsonWebTokenService>(jwt_service.into())
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

            auth::JsonWebTokenService,

            auth::OAuthRepository,
            auth::GoogleOAuthService,

            auth::RedisSessionRepository,
            auth::SessionServiceImpl,

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
