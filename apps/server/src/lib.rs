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

    pub use entity::{
        Administrator, Role, Secretary, Student, Teacher, User, Users,
    };
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

pub mod shared;
