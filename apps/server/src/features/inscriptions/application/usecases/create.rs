use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::{
    asignatures::domain::AsignatureRepository,
    inscriptions::{
        application::CreateInscriptionCase,
        domain::{
            Inscription, InscriptionError, InscriptionFilter, InscriptionRepository,
        },
    },
};

use crate::users::domain::UserRepository;

#[derive(Component)]
#[shaku(interface = CreateInscriptionCase)]
pub struct CreateInscriptionCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn InscriptionRepository>,

    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,

    #[shaku(inject)]
    asignature_repository: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl CreateInscriptionCase for CreateInscriptionCaseImpl {
    async fn execute(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError> {
        let user_id = inscription.user_id.clone();

        let filter = InscriptionFilter {
            user_id: Some(inscription.user_id),
            asignature_id: Some(inscription.asignature_id),
            status: None,
        };

        if !self.repository.find_all(filter).await?.is_empty() {
            return Err(InscriptionError::InscriptionAlreadyExists);
        }

        let (user_exists, asignature_exists) = tokio::join!(
            self.user_repository.find_by_id(&user_id),
            self.asignature_repository
                .find_by_id(&inscription.asignature_id)
        );

        let Some(user) = user_exists? else {
            return Err(InscriptionError::StudentNotFound { id: user_id });
        };

        if asignature_exists?.is_none() {
            return Err(InscriptionError::AsignatureNotFound {
                id: inscription.asignature_id,
            });
        };

        if !user.roles.contains(&"student".to_string()) {
            return Err(InscriptionError::InvalidStudentRole);
        }

        self.repository.create(inscription).await
    }
}
