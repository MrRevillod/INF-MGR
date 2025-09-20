use crate::shared::AppError;

use async_trait::async_trait;
use shaku::{Component, Interface};

#[derive(Component)]
#[shaku(interface = ImportService)]
pub struct ImportServiceImpl {}

#[async_trait]
pub trait ImportService: Interface {
    async fn import_course_students(&self) -> Result<(), AppError>;
}

#[async_trait]
impl ImportService for ImportServiceImpl {
    async fn import_course_students(&self) -> Result<(), AppError> {
        Ok(())
    }
}
