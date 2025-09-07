use shaku::module;

use crate::auth::JsonWebTokenService;
use crate::shared::database::PostgresDatabase;
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

module! {
    pub AppModule {
        components = [
            RedisDatabase,
            PostgresDatabase,
            GoogleOAuthClient,

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
