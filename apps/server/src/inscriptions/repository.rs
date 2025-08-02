use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    inscriptions::{entity::Inscription, errors::InscriptionError},
    shared::database::DatabaseConnection,
};

#[derive(Component)]
#[shaku(interface = InscriptionRepository)]
pub struct PostgresInscriptionRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[derive(Default)]
pub struct InscriptionFilter {
    pub student_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
}

#[async_trait]
pub trait InscriptionRepository: Interface {
    async fn find_all(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<Inscription>, InscriptionError>;

    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Inscription>, InscriptionError>;

    async fn save(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError>;

    async fn delete(&self, id: &Uuid) -> Result<(), InscriptionError>;
}

#[async_trait]
impl InscriptionRepository for PostgresInscriptionRepository {
    async fn find_all(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<Inscription>, InscriptionError> {
        let mut builder = sqlx::QueryBuilder::<Postgres>::new(
            "SELECT * FROM inscriptions WHERE 1=1",
        );

        if let Some(user_id) = filter.student_id {
            builder.push(" AND student_id = ").push_bind(user_id);
        }

        if let Some(course_id) = filter.course_id {
            builder.push(" AND course_id = ").push_bind(course_id);
        }

        let query = builder.build_query_as::<Inscription>();
        let result = query.fetch_all(self.db_connection.get_pool()).await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Inscription>, InscriptionError> {
        let query = r#"SELECT * FROM inscriptions WHERE id = $1"#;

        let model = sqlx::query_as::<_, Inscription>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model)
    }

    async fn save(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError> {
        let query = r#"
            INSERT INTO inscriptions (id, student_id, course_id, practice_id, student_scores)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                student_id = EXCLUDED.student_id,
                course_id = EXCLUDED.course_id,
                student_scores = EXCLUDED.student_scores
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, Inscription>(query)
            .bind(inscription.id)
            .bind(inscription.student_id)
            .bind(inscription.course_id)
            .bind(inscription.practice_id)
            .bind(inscription.student_scores)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), InscriptionError> {
        sqlx::query("DELETE FROM inscriptions WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
