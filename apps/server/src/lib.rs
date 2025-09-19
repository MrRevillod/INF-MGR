pub mod config;

pub mod auth {
    mod controllers;
    mod dtos;
    mod middlewares {
        mod authentication;
        pub use authentication::Authentication;

        mod role;
        pub use role::{MinimumRequiredRole, OwnershipValidation};
    }

    pub use middlewares::Authentication;
    pub use middlewares::{MinimumRequiredRole, OwnershipValidation};

    mod repositories {
        mod auth;
        mod session;

        pub use auth::{AuthRepository, OAuthRepository};
        pub use session::{RedisSessionRepository, SessionRepository};
    }

    mod services {
        mod oauth;
        pub use oauth::{GoogleOAuthService, OAuthService};

        mod session;
        pub use session::{SessionService, SessionServiceImpl};

        mod jsonwebtoken;
        pub use jsonwebtoken::{
            Claims, JsonWebTokenService, TokenConfig, TokenKind, TokenService,
        };

        mod cookies;

        pub use cookies::CookieBuilder;
    }

    pub use services::{Claims, TokenConfig, TokenKind, TokenService};

    pub use controllers::AuthController;
    pub use repositories::{
        AuthRepository, OAuthRepository, RedisSessionRepository, SessionRepository,
    };

    pub use dtos::*;
    pub use services::{
        GoogleOAuthService, JsonWebTokenService, OAuthService, SessionService,
        SessionServiceImpl,
    };

    mod entity;
    pub use entity::Session;
}

pub mod users {
    mod controllers;
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use controllers::UsersController;
    pub use dtos::{
        CreateUserDto, GetUsersQueryDto, UpdateUserDto, UserResponse,
        role_validator, validate_rut_id,
    };

    pub use entity::{Role, User};
    pub use repository::{PostgresUserRepository, UserFilter, UserRepository};
    pub use service::{UserService, UserServiceImpl};

    pub use crate::user_filter;
}

pub mod courses {
    mod controllers;
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use crate::course_filter;
    pub use controllers::CoursesController;
    pub use dtos::{
        CourseEvaluationDto, CourseResponse, CourseWithStaff, CreateCourseDto,
        UpdateCourseDto,
    };
    pub use entity::{Course, CourseEvaluation, CourseStatus};
    pub use repository::{CourseFilter, CourseRepository, PostgresCourseRepository};
    pub use service::{CourseService, CourseServiceImpl};
}

pub mod imports {
    mod controllers;
    mod dtos;
    mod service;

    pub use controllers::ImportsController;
    pub use dtos::{ImportCourseDto, ImportUserDto, ImportedUser};
    pub use service::{ImportService, ImportServiceImpl};
}

pub mod enrollments {
    mod controllers;
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use crate::enrollment_filter;
    pub use controllers::EnrollmentsController;
    pub use dtos::{
        CreateEnrollmentDto, EnrollmentResponse, EnrollmentWithStudentAndPractice,
        GetEnrollmentsDto, StudentScoreDto, UpdateEnrollmentDto,
    };

    pub use entity::{Enrollment, StudentScore};
    pub use repository::{
        EnrollmentFilter, EnrollmentRepository, PostgresEnrollmentRepository,
    };
    pub use service::{EnrollmentService, EnrollmentServiceImpl};
}

pub mod practices {
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use crate::practice_filter;
    pub use dtos::{CreatePracticeDto, EvaluatePracticeDto, UpdatePracticeDto};
    pub use entity::{Practice, PracticeStatus, Practices};
    pub use repository::{
        PostgresPracticeRepository, PracticeFilter, PracticeRepository,
    };
    pub use service::{PracticeService, PracticeServiceImpl};
}

pub mod shared {
    pub mod errors;
    pub use errors::{AppError, AppResult};
    use sword::__internal::IntoResponse;
    use sword::web::StatusCode;

    pub mod di {
        mod builder;
        mod container;

        pub use builder::DependencyContainer;
        pub use container::{AppModule, InitialComponents};
    }

    pub mod macros;

    pub mod context;
    pub mod database;
    pub mod layers;
    pub mod oauth;
    pub mod redis;

    pub mod validators {
        use validator::ValidationError;

        pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
            if uuid.is_empty() {
                return Err(ValidationError::new(
                    "La identificación no puede estar vacía.",
                ));
            }

            if uuid::Uuid::parse_str(uuid).is_err() {
                return Err(ValidationError::new("Identificación inválida."));
            }

            Ok(())
        }
    }

    pub mod services {
        pub mod errors;
        pub mod mailer;
        pub mod printer;
        pub mod templates {
            mod context;
            mod files;

            pub use context::*;
            pub use files::*;
        }

        pub mod event_queue {
            mod publisher;
            mod subscriber;

            mod events;

            pub use events::*;
            pub use publisher::*;
            pub use subscriber::*;
        }
    }

    pub enum FileResponse {
        Document(Vec<u8>),
    }

    impl IntoResponse for FileResponse {
        fn into_response(self) -> axum::response::Response {
            match self {
                FileResponse::Document(data) => {
                    (StatusCode::OK, [("Content-type", "application/pdf")], data)
                        .into_response()
                }
            }
        }
    }
}
