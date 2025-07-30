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
    inscriptions: Arc<dyn InscriptionRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    asignatures: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl CreateInscriptionCase for CreateInscriptionCaseImpl {
    async fn execute(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError> {
        let user_id = inscription.user_id;

        let filter = InscriptionFilter {
            user_id: Some(inscription.user_id),
            asignature_id: Some(inscription.asignature_id),
            status: None,
        };

        if !self.inscriptions.find_all(filter).await?.is_empty() {
            return Err(InscriptionError::InscriptionAlreadyExists);
        }

        let (user_exists, asignature_exists) = tokio::join!(
            self.users.find_by_id(&user_id),
            self.asignatures.find_by_id(&inscription.asignature_id)
        );

        let user_exists = user_exists
            .map_err(|e| InscriptionError::ForeignUserError(e.to_string()))?;

        let asignature_exists = asignature_exists
            .map_err(|e| InscriptionError::ForeignAsignatureError(e.to_string()))?;

        let Some(user) = user_exists else {
            return Err(InscriptionError::StudentNotFound { id: user_id });
        };

        if asignature_exists.is_none() {
            return Err(InscriptionError::AsignatureNotFound {
                id: inscription.asignature_id,
            });
        };

        if !user.roles.contains(&"student".to_string()) {
            return Err(InscriptionError::InvalidStudentRole);
        }

        self.inscriptions.create(inscription).await
    }
}
