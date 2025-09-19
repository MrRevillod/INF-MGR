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

    pub use entity::{Role, User, Users};
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
    pub use entity::{Course, CourseEvaluation, CourseStatus, Courses};
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

    pub use entity::{Enrollment, Enrollments, StudentScore};
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
    pub use errors::{AppError, AppResult, AuthError, Input};
    pub mod macros;

    mod infrastructure {

        pub mod di {
            mod builder;
            mod container;

            pub use builder::DependencyContainer;
            pub use container::{AppModule, InitialComponents};
        }

        mod database;
        mod http;
        mod layers;
        mod oauth;
        mod redis;
        mod uuid;

        pub use database::*;
        pub use di::*;
        pub use http::*;
        pub use layers::*;
        pub use oauth::*;
        pub use redis::*;
        pub use uuid::validate_uuid;
    }

    pub use infrastructure::*;

    pub mod services {
        mod errors;
        mod mailer;
        mod printer;
        mod templates {
            mod context;
            mod files;

            pub use context::*;
            pub use files::*;
        }

        mod event_queue {
            mod publisher;
            mod subscriber;

            mod events;

            pub use events::*;
            pub use publisher::*;
            pub use subscriber::*;
        }

        pub use errors::*;
        pub use event_queue::*;
        pub use mailer::*;
        pub use printer::*;
        pub use templates::*;
    }
}
