use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::inscriptions::{
    application::GetInscriptionsCase,
    domain::{
        Inscription, InscriptionError, InscriptionFilter, InscriptionRepository,
    },
};

#[derive(Component)]
#[shaku(interface = GetInscriptionsCase)]
pub struct GetInscriptionsCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn InscriptionRepository>,
}

#[async_trait]
impl GetInscriptionsCase for GetInscriptionsCaseImpl {
    async fn execute(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<Inscription>, InscriptionError> {
        self.repository.find_all(filter).await
    }
}
