use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::inscriptions::{
    application::CreateInscriptionCase,
    domain::{
        Inscription, InscriptionError, InscriptionFilter, InscriptionRepository,
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

        let user_exists = self.user_repository.find_by_id(&user_id).await?;

        let Some(user) = user_exists else {
            return Err(InscriptionError::StudentNotFound { id: user_id });
        };

        if !user.roles.contains(&"student".to_string()) {
            return Err(InscriptionError::InvalidStudentRole);
        }

        self.repository.create(inscription).await
    }
}
