use async_trait::async_trait;
use shaku::Component;
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

use crate::inscriptions::{
    domain::{
        Inscription, InscriptionError, InscriptionFilter, InscriptionRepository,
    },
    infrastructure::models::{InscriptionModel, StudentEvaluationModel},
};

use crate::shared::database::DatabaseConnection;

#[derive(Component)]
#[shaku(interface = InscriptionRepository)]
pub struct PostgresInscriptionRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
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

        if let Some(user_id) = filter.user_id {
            builder.push(" AND user_id = ").push_bind(user_id);
        }

        if let Some(asignature_id) = filter.asignature_id {
            builder
                .push(" AND asignature_id = ")
                .push_bind(asignature_id);
        }

        if let Some(status) = filter.status {
            builder.push(" AND status = ").push_bind(status);
        }

        let query = builder.build_query_as::<InscriptionModel>();
        let result = query.fetch_all(self.db_connection.get_pool()).await?;

        Ok(result.into_iter().map(Inscription::from).collect())
    }

    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Inscription>, InscriptionError> {
        let query = r#"SELECT * FROM inscriptions WHERE id = $1"#;

        let model = sqlx::query_as::<_, InscriptionModel>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model.map(Inscription::from))
    }

    async fn create(
        &self,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError> {
        let query = r#"
            INSERT INTO inscriptions (id, user_id, asignature_id, practice_id, evaluations_scores, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#;

        let scores: Vec<StudentEvaluationModel> = inscription
            .evaluation_scores
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        let result = sqlx::query_as::<_, InscriptionModel>(query)
            .bind(inscription.id)
            .bind(inscription.user_id)
            .bind(inscription.asignature_id)
            .bind(inscription.practice_id)
            .bind(scores)
            .bind(inscription.status)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(result.into())
    }

    async fn update(
        &self,
        id: &Uuid,
        inscription: Inscription,
    ) -> Result<Inscription, InscriptionError> {
        let query = r#"
            UPDATE inscriptions SET evaluations_scores = $1, status = $2
            WHERE id = $3
            RETURNING *
        "#;

        let scores: Vec<StudentEvaluationModel> = inscription
            .evaluation_scores
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        let result = sqlx::query_as::<_, InscriptionModel>(query)
            .bind(scores)
            .bind(inscription.status)
            .bind(id)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(result.into())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), InscriptionError> {
        sqlx::query("DELETE FROM inscriptions WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
