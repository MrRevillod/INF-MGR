pub mod config;

pub mod users {
    mod controllers;
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use controllers::UsersController;
    pub use dtos::{
        role_validator, validate_rut_id, CreateUserDto, GetUsersQueryDto, UpdateUserDto,
        UserResponse,
    };
    pub use entity::{Role, User};
    pub use repository::{PostgresUserRepository, UserFilter, UserRepository};

    pub use service::{UserService, UserServiceImpl};
}

pub mod courses {
    mod controllers;
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use controllers::CoursesController;
    pub use dtos::{
        CourseEvaluationDto, CourseResponse, CourseWithStaff, CreateCourseDto, UpdateCourseDto,
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

    pub use controllers::EnrollmentsController;
    pub use dtos::{
        CreateEnrollmentDto, EnrollmentResponse, EnrollmentWithStudentAndPractice,
        GetEnrollmentsDto, StudentScoreDto, UpdateEnrollmentDto,
    };

    pub use entity::{Enrollment, StudentScore};

    pub use repository::{EnrollmentFilter, EnrollmentRepository, PostgresEnrollmentRepository};

    pub use service::{EnrollmentService, EnrollmentServiceImpl};
}

pub mod practices {
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use dtos::{CreatePracticeDto, UpdatePracticeDto};
    pub use entity::{Practice, PracticeStatus, Practices};
    pub use repository::{PostgresPracticeRepository, PracticeFilter, PracticeRepository};
    pub use service::{PracticeService, PracticeServiceImpl};
}

pub mod shared {
    pub mod entities;
    pub mod errors;

    pub use errors::{AppError, AppResult};

    pub mod macros;

    pub mod database;
    pub mod layers;
    pub mod validators {
        use validator::ValidationError;

        pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
            if uuid.is_empty() {
                return Err(ValidationError::new("La identificación no puede estar vacía."));
            }

            if uuid::Uuid::parse_str(uuid).is_err() {
                return Err(ValidationError::new("Identificación inválida."));
            }

            Ok(())
        }
    }

    pub mod services {
        pub mod errors;
        pub mod hasher;
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
            mod sender;
            mod subscriber;

            mod events;

            pub use events::*;
            pub use publisher::*;
            pub use sender::*;
            pub use subscriber::*;
        }
    }
}

pub mod container;
