use shaku::module;

use crate::shared::services::*;
use crate::shared::{GoogleOAuthClient, PostgresDatabase, RedisDatabase};

use crate::{auth, courses, enrollments, imports, meetings, practices, users};

pub type InitialComponents = (
    PostgresDatabase,
    Mailer,
    Printer,
    GoogleOAuthClient,
    RedisDatabase,
    auth::JsonWebTokenService,
    CalendarHub,
);

module! {
    pub AppModule {
        components = [
            RedisDatabase,
            PostgresDatabase,
            GoogleOAuthClient,
            CalendarHub,

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

            meetings::PostgresMeetingRepository,
            meetings::MeetingServiceImpl,

            imports::ImportServiceImpl,

            practices::PostgresPracticeRepository,
            practices::PracticeServiceImpl,

            meetings::PostgresMeetingRequestsRepository,
            meetings::MeetingRequestServiceImpl,
        ],
        providers = []
    }
}
