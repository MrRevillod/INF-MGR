use async_trait::async_trait;
use shaku::Component;
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    asignatures::{
        domain::{
            Asignature, AsignatureError, AsignatureFilter, AsignatureRepository,
        },
        infrastructure::models::{AsignatureModel, EvaluationType},
    },
    shared::database::DatabaseConnection,
};

#[derive(Component)]
#[shaku(interface = AsignatureRepository)]
pub struct PostgresAsignatureRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
impl AsignatureRepository for PostgresAsignatureRepository {
    async fn find_all(&self) -> Result<Vec<Asignature>, AsignatureError> {
        let pool = self.db_connection.get_pool();
        let query = r#"SELECT * FROM asignatures"#;

        let result = sqlx::query_as::<_, AsignatureModel>(query)
            .fetch_all(pool)
            .await
            .map_err(|e| AsignatureError::DatabaseError(e.to_string()))?;

        Ok(result.into_iter().map(Asignature::from).collect())
    }

    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Asignature>, AsignatureError> {
        let query = r#"SELECT * FROM asignatures WHERE id = $1"#;

        let model = sqlx::query_as::<_, AsignatureModel>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await
            .map_err(|e| AsignatureError::DatabaseError(e.to_string()))?;

        Ok(model.map(Asignature::from))
    }

    async fn find_by_filter(
        &self,
        filter: AsignatureFilter,
    ) -> Result<Vec<Asignature>, AsignatureError> {
        let mut builder = sqlx::QueryBuilder::<Postgres>::new(
            "SELECT * FROM asignatures WHERE 1=1",
        );

        if let Some(year) = filter.year {
            builder.push(" AND year = ").push_bind(year);
        }

        if let Some(ref code) = filter.code {
            builder.push(" AND code = ").push_bind(code);
        }

        if let Some(ref name) = filter.name {
            builder.push(" AND name = ").push_bind(name);
        }

        let query = builder.build_query_as::<AsignatureModel>();

        let result = query
            .fetch_all(self.db_connection.get_pool())
            .await
            .map_err(|e| AsignatureError::DatabaseError(e.to_string()))?;

        Ok(result.into_iter().map(Asignature::from).collect())
    }

    async fn create(
        &self,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError> {
        let query = r#"
            INSERT INTO asignatures (id, year, code, name, evaluations, teacher_id) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING *
        "#;

        let evaluations: Vec<EvaluationType> = input
            .evaluations
            .into_iter()
            .map(EvaluationType::from)
            .collect();

        let model = sqlx::query_as::<_, AsignatureModel>(query)
            .bind(input.id)
            .bind(input.year)
            .bind(input.code)
            .bind(input.name)
            .bind(evaluations)
            .bind(input.teacher_id)
            .fetch_one(self.db_connection.get_pool())
            .await
            .map_err(|e| AsignatureError::DatabaseError(e.to_string()))?;

        Ok(Asignature::from(model))
    }

    async fn update(
        &self,
        id: &Uuid,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError> {
        let query = r#"
            UPDATE asignatures 
            SET year = $1, code = $2, name = $3, evaluations = $4, teacher_id = $5 
            WHERE id = $6 
            RETURNING *
        "#;

        let evaluations: Vec<EvaluationType> = input
            .evaluations
            .into_iter()
            .map(EvaluationType::from)
            .collect();

        let model: AsignatureModel = sqlx::query_as(query)
            .bind(input.year)
            .bind(input.code)
            .bind(input.name)
            .bind(evaluations)
            .bind(input.teacher_id)
            .bind(id)
            .fetch_one(self.db_connection.get_pool())
            .await
            .map_err(|e| AsignatureError::DatabaseError(e.to_string()))?;

        Ok(Asignature::from(model))
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AsignatureError> {
        let query = r#"DELETE FROM asignatures WHERE id = $1"#;

        sqlx::query(query)
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await
            .map_err(|e| AsignatureError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
