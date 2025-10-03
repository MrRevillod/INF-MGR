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
