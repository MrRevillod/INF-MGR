use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;
use uuid::Uuid;

use crate::{
    practices::{
        domain::{Practice, PracticeError, PracticeRepository},
        infrastructure::models::PracticeModel,
    },
    shared::database::DatabaseConnection,
};

#[derive(Component)]
#[shaku(interface = PracticeRepository)]
pub struct PostgresPracticeRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
impl PracticeRepository for PostgresPracticeRepository {
    async fn create(&self, practice: Practice) -> Result<Practice, PracticeError> {
        let query = r#"
            INSERT INTO practices (
                id, enterprise_name, location, description, 
                supervisor_name, supervisor_email, start_date, end_date
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;

        sqlx::query(query)
            .bind(practice.id)
            .bind(&practice.enterprise_name)
            .bind(&practice.location)
            .bind(&practice.description)
            .bind(&practice.supervisor_name)
            .bind(&practice.supervisor_email)
            .bind(practice.start_date)
            .bind(practice.end_date)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(practice)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Practice>, PracticeError> {
        let query = "SELECT * FROM practices WHERE id = $1";
        let row = sqlx::query_as::<_, PracticeModel>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(row.map(Practice::from))
    }

    async fn update(&self, practice: Practice) -> Result<Practice, PracticeError> {
        let query = r#"
            UPDATE practices SET 
                enterprise_name = $1, 
                location = $2, 
                description = $3, 
                supervisor_name = $4, 
                supervisor_email = $5, 
                start_date = $6, 
                end_date = $7
            WHERE id = $8
        "#;

        sqlx::query(query)
            .bind(&practice.enterprise_name)
            .bind(&practice.location)
            .bind(&practice.description)
            .bind(&practice.supervisor_name)
            .bind(&practice.supervisor_email)
            .bind(practice.start_date)
            .bind(practice.end_date)
            .bind(practice.id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(practice)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), PracticeError> {
        sqlx::query("DELETE FROM practices WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
