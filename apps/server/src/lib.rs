pub mod config;

pub mod users {
    mod controllers;
    mod dtos;
    mod entity;
    mod repository;
    mod service;

    pub use controllers::UsersController;
    pub use dtos::{CreateUserDto, GetUsersQueryDto, UpdateUserDto, UserResponse};
    pub use entity::{Role, User};
    pub use repository::{
        PostgresUserRepository, UserFilter, UserRepository, UserWithCount,
    };

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
        CourseEvaluationDto, CourseResponse, CourseWithStaff, CreateCourseDto,
        UpdateCourseDto,
    };

    pub use entity::{Course, CourseEvaluation, CourseStatus};

    pub use repository::{CourseFilter, CourseRepository, PostgresCourseRepository};
    pub use service::{CourseService, CourseServiceImpl};
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

    pub use dtos::{CreatePracticeDto, UpdatePracticeDto};
    pub use entity::Practice;
    pub use repository::{PostgresPracticeRepository, PracticeRepository};
    pub use service::{PracticeService, PracticeServiceImpl};
}

pub mod shared {
    pub mod entities;
    pub mod errors;

    use chrono::{DateTime, Utc};
    use chrono_tz::America::Santiago;
    pub use errors::{AppError, AppResult};

    pub mod database;
    pub mod layers;
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

    pub fn format_date(date: Option<DateTime<Utc>>) -> String {
        date.map(|date| date.with_timezone(&Santiago).format("%d/%m/%y").to_string())
            .unwrap_or_default()
    }
}

pub mod container;
