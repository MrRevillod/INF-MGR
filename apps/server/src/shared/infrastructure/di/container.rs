use shaku::module;

use crate::shared::services::*;
use crate::shared::{GoogleOAuthClient, PostgresDatabase, RedisDatabase};

use crate::{auth, courses, enrollments, imports, practices, users};

pub type InitialComponents = (
    PostgresDatabase,
    Mailer,
    Printer,
    GoogleOAuthClient,
    RedisDatabase,
    auth::JsonWebTokenService,
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
