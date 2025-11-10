mod dtos;
mod entity;
mod repository;
mod services;

pub use crate::practice_filter;
pub use dtos::{CreatePracticeDto, EvaluatePracticeDto, UpdatePracticeDto};
pub use entity::{Practice, PracticeStatus};
pub use repository::{PracticeFilter, PracticeRepository};
pub use services::{PracticeReportService, PracticeService};

use sword::prelude::*;

pub struct PracticesModule;

impl Module for PracticesModule {
    type Controller = NonControllerModule;

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<PracticeService>();
        container.register_component::<PracticeRepository>();
        container.register_component::<PracticeReportService>();
    }
}
