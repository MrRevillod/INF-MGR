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
pub use repository::{CourseFilter, CourseRepository};
pub use service::CourseService;

use sword::prelude::*;

pub struct CoursesModule;

impl Module for CoursesModule {
    type Controller = CoursesController;

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<CourseRepository>();
        container.register_component::<CourseService>();
    }
}
