use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::{
    asignatures::{
        application::CreateAsignatureCase,
        domain::{
            Asignature, AsignatureError, AsignatureFilter, AsignatureRepository,
        },
    },
    users::domain::UserRepository,
};

#[derive(Component)]
#[shaku(interface = CreateAsignatureCase)]
pub struct CreateAsignatureCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn AsignatureRepository>,

    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl CreateAsignatureCase for CreateAsignatureCaseImpl {
    async fn execute(
        &self,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError> {
        let filter = AsignatureFilter {
            year: Some(input.year),
            code: Some(input.code.clone()),
            name: Some(input.name.clone()),
        };

        if !self.repository.find_by_filter(filter).await?.is_empty() {
            return Err(AsignatureError::AlreadyExists);
        }

        let user_exists = self
            .user_repository
            .find_by_id(&input.teacher_id)
            .await
            .map_err(|_| {
            AsignatureError::UnexpectedError(
                "Error interno del servidor".to_string(),
            )
        })?;

        let Some(user) = user_exists else {
            return Err(AsignatureError::TeacherNotFound);
        };

        if user.role.as_str() != "teacher" {
            return Err(AsignatureError::UserIsNotTeacher);
        }

        Ok(self.repository.create(input).await?)
    }
}
