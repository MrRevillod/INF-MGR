mod controllers;
mod dtos;
mod entity;
mod repository;
mod service;

pub use crate::enrollment_filter;
pub use controllers::EnrollmentsController;
pub use dtos::{
    CreateEnrollmentDto, EnrollmentResponse,
    EnrollmentWithStudentAndPracticeAndCourse, GetEnrollmentsDto, StudentScoreDto,
    UpdateEnrollmentDto,
};

pub use entity::{Enrollment, StudentScore};
pub use repository::{EnrollmentFilter, EnrollmentRepository};
pub use service::EnrollmentService;

use sword::prelude::*;

pub struct EnrollmentsModule;

impl Module for EnrollmentsModule {
    type Controller = EnrollmentsController;

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<EnrollmentRepository>();
        container.register_component::<EnrollmentService>();
    }
}
