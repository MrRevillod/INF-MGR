use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    asignatures::domain::{Asignature, AsignatureRepository},
    inscriptions::domain::{Inscription, InscriptionFilter, InscriptionRepository},
    users::{
        application::interfaces::GetUsersCase,
        domain::{
            FindAllReturnType, GetUsersParams, User, UserError, UserRepository,
        },
    },
};

#[derive(Component)]
#[shaku(interface = GetUsersCase)]
pub struct GetUsersCaseImpl {
    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    inscriptions: Arc<dyn InscriptionRepository>,

    #[shaku(inject)]
    asignatures: Arc<dyn AsignatureRepository>,
}

#[async_trait]
impl GetUsersCase for GetUsersCaseImpl {
    async fn get_all(
        &self,
        filter: GetUsersParams,
    ) -> Result<FindAllReturnType, UserError> {
        self.users.find_all(filter).await
    }

    async fn get_student_inscriptions(
        &self,
        student_id: &Uuid,
    ) -> Result<Vec<(Asignature, Inscription)>, UserError> {
        let Some(user) = self.users.find_by_id(student_id).await? else {
            return Err(UserError::NotFound);
        };

        if !user.is_student() {
            return Err(UserError::InvalidRole {
                role: user.roles.get(0).unwrap_or(&"unknown".to_string()).clone(),
            });
        }

        let inscription_filter = InscriptionFilter {
            user_id: Some(student_id.clone()),
            ..Default::default()
        };

        let inscriptions = self
            .inscriptions
            .find_all(inscription_filter)
            .await
            .map_err(|e| UserError::ForeignInscriptionError(e.to_string()))?;

        let mut result = Vec::new();

        for inscription in inscriptions {
            let asignature = self
                .asignatures
                .find_by_id(&inscription.asignature_id)
                .await
                .map_err(|e| UserError::ForeignAsignatureError(e.to_string()))?;

            if let Some(asignature) = asignature {
                result.push((asignature, inscription));
            }
        }

        Ok(result)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<User, UserError> {
        match self.users.find_by_id(user_id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(UserError::NotFound),

            Err(e) => Err(e),
        }
    }
}
