mod controllers;
mod dtos;
mod service;

pub use controllers::ImportsController;
pub use dtos::*;
pub use service::ImportService;

use sword::prelude::*;

pub struct ImportsModule;

impl Module for ImportsModule {
    type Controller = ImportsController;

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<ImportService>();
    }
}
