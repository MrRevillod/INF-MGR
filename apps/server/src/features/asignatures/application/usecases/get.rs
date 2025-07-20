use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::asignatures::{
    application::GetAsignaturesCase,
    domain::{Asignature, AsignatureError, AsignatureRepository},
};

#[derive(Component)]
#[shaku(interface = GetAsignaturesCase)]
pub struct GetAsignaturesCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl GetAsignaturesCase for GetAsignaturesCaseImpl {
    async fn execute(&self) -> Result<Vec<Asignature>, AsignatureError> {
        self.repository.find_all().await
    }
}
