pub mod config;

pub mod users {

    mod controllers;
    mod dtos;
    mod entity;
    mod errors;
    mod repository;
    mod service;

    pub use controllers::UsersController;
    pub use dtos::{CreateUserDto, GetUsersQueryDto, UpdateUserDto, UserResponse};
    pub use entity::{Role, User};
    pub use errors::UserError;
    pub use repository::{
        PostgresUserRepository, UserFilter, UserRepository, UserWithCount,
    };
    pub use service::{UserService, UserServiceImpl};
}

pub mod courses {

    mod controllers;
    mod dtos;
    mod entity;
    mod errors;
    mod repository;
    mod service;

    pub use controllers::CoursesController;
    pub use dtos::{
        CourseEvaluationDto, CourseResponse, CourseWithStaff, CreateCourseDto,
        UpdateCourseDto,
    };
    pub use entity::{Course, CourseEvaluation, CourseStatus};
    pub use errors::CourseError;

    pub use repository::{CourseFilter, CourseRepository, PostgresCourseRepository};
    pub use service::{CourseService, CourseServiceImpl};
}

pub mod enrollments {

    mod dtos;
    mod entity;
    mod errors;
    mod repository;
    mod service;

    pub use dtos::{
        CreateEnrollmentDto, EnrollmentResponse, EnrollmentWithStudent,
        GetEnrollmentsDto, StudentScoreDto, UpdateEnrollmentDto,
    };

    pub use entity::{Enrollment, StudentScore};
    pub use errors::EnrollmentError;

    pub use repository::{
        EnrollmentFilter, EnrollmentRepository, PostgresEnrollmentRepository,
    };

    pub use service::{EnrollmentService, EnrollmentServiceImpl};
}

pub mod shared {
    pub mod services {
        pub mod errors;
        mod mailer;
        mod password;
        pub use mailer::{MailContext, MailTo, Mailer, MailerService};
        pub use password::{BcryptPasswordHasher, PasswordHasher};
    }

    pub mod entities;

    pub mod database;
    pub mod di;
    pub mod layers;
    pub mod smtp;
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
}
