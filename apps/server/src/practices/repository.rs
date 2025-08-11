use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};
use uuid::Uuid;

use crate::{
    practices::entity::Practice,
    shared::{database::DatabaseConnection, errors::AppError},
};

#[derive(Component)]
#[shaku(interface = PracticeRepository)]
pub struct PostgresPracticeRepository {
    #[shaku(inject)]
    pub db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
pub trait PracticeRepository: Interface {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Practice>, AppError>;
    async fn save(&self, practice: Practice) -> Result<Practice, AppError>;
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl PracticeRepository for PostgresPracticeRepository {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError> {
        let query = "SELECT * FROM practices WHERE id = $1";
        let practice = sqlx::query_as::<_, Practice>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(practice)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Practice>, AppError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let query = "SELECT * FROM practices WHERE id = ANY($1)";
        let practices = sqlx::query_as::<_, Practice>(query)
            .bind(ids)
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(practices)
    }

    async fn save(&self, practice: Practice) -> Result<Practice, AppError> {
        let query = "
            INSERT INTO practices (
                id, enterprise_name, location,
                description, supervisor_name, supervisor_email,
                start_date, end_date, is_approved
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                enterprise_name = EXCLUDED.enterprise_name,
                location = EXCLUDED.location,
                description = EXCLUDED.description,
                supervisor_name = EXCLUDED.supervisor_name,
                supervisor_email = EXCLUDED.supervisor_email,
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                is_approved = EXCLUDED.is_approved
            RETURNING *";

        let saved_practice = sqlx::query_as::<_, Practice>(query)
            .bind(practice.id)
            .bind(practice.enterprise_name)
            .bind(practice.location)
            .bind(practice.description)
            .bind(practice.supervisor_name)
            .bind(practice.supervisor_email)
            .bind(practice.start_date)
            .bind(practice.end_date)
            .bind(practice.is_approved)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(saved_practice)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM practices WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
