use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error(
        "Invalid role: '{value}' (expected: student, teacher, secretary, admin)"
    )]
    InvalidRole { value: String },

    #[error("Invalid course status: '{value}' (expected: active, completed)")]
    InvalidCourseStatus { value: String },

    #[error("Invalid MeetingStatus: {0}")]
    InvalidMeetingStatus(String),

    #[error("Invalid datetime: {0}")]
    InvalidDatetime(String),

    // === Business Logic Validation (400, not 409) ===
    #[error("User {user_id} is not a student")]
    NotAStudent { user_id: Uuid },

    #[error("User {user_id} is not a teacher")]
    NotATeacher { user_id: Uuid },

    #[error("Cannot delete course: has active enrollments")]
    CourseHasEnrollments { course_id: Uuid },

    #[error("Cannot upload final report: enrollment has no associated practice")]
    NoPracticeAssociated,

    #[error("Final report upload period has expired")]
    FinalReportUploadExpired,

    #[error("Cannot create meeting request with less than 2 attendees")]
    NotEnoughAttendees,

    // === Constraint Violations (400, not 409) ===
    #[error("Email '{email}' is already in use")]
    DuplicateEmail { email: String },

    #[error("RUT '{rut}' is already in use")]
    DuplicateRut { rut: String },

    #[error("Student is already enrolled in this course")]
    DuplicateEnrollment { student_id: Uuid, course_id: Uuid },

    #[error("Course with same code/name already exists for this year")]
    DuplicateCourse {
        code: String,
        name: String,
        year: i32,
    },

    // === Related Entity Validation (400, not 404) ===
    #[error("Invalid student ID: {student_id}")]
    InvalidStudentId { student_id: Uuid },

    #[error("Invalid course ID: {course_id}")]
    InvalidCourseId { course_id: Uuid },

    #[error("Invalid teacher ID: {teacher_id}")]
    InvalidTeacherId { teacher_id: Uuid },

    #[error("Invalid Tex structure in uploaded final report: {message}")]
    InvalidTexStructure { message: String },
}

impl ValidationError {
    // === Parsing/Format Helpers ===
    pub fn invalid_role(value: impl Into<String>) -> Self {
        Self::InvalidRole {
            value: value.into(),
        }
    }

    pub fn invalid_course_status(value: impl Into<String>) -> Self {
        Self::InvalidCourseStatus {
            value: value.into(),
        }
    }

    // === Business Logic Helpers ===
    pub fn not_a_student(user_id: Uuid) -> Self {
        Self::NotAStudent { user_id }
    }

    pub fn not_a_teacher(user_id: Uuid) -> Self {
        Self::NotATeacher { user_id }
    }

    pub fn course_has_enrollments(course_id: Uuid) -> Self {
        Self::CourseHasEnrollments { course_id }
    }

    // === Constraint Violation Helpers ===
    pub fn duplicate_email(email: impl Into<String>) -> Self {
        Self::DuplicateEmail {
            email: email.into(),
        }
    }

    pub fn duplicate_rut(rut: impl Into<String>) -> Self {
        Self::DuplicateRut { rut: rut.into() }
    }

    pub fn duplicate_enrollment(student_id: Uuid, course_id: Uuid) -> Self {
        Self::DuplicateEnrollment {
            student_id,
            course_id,
        }
    }

    pub fn duplicate_course(
        name: impl Into<String>,
        year: i32,
        code: impl Into<String>,
    ) -> Self {
        Self::DuplicateCourse {
            code: code.into(),
            name: name.into(),
            year,
        }
    }

    pub fn invalid_datetime(value: impl Into<String>) -> Self {
        Self::InvalidDatetime(value.into())
    }

    pub fn invalid_tex_structure(message: impl Into<String>) -> Self {
        Self::InvalidTexStructure {
            message: message.into(),
        }
    }
}
