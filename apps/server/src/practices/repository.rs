use std::sync::Arc;

use async_trait::async_trait;
use sea_query::{Expr, ExprTrait, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{query_as_with as sqlx_query, Postgres};
use uuid::Uuid;

use crate::{
    practices::entity::{Practice, Practices},
    shared::{database::DatabaseConnection, errors::AppError},
};

#[derive(Component)]
#[shaku(interface = PracticeRepository)]
pub struct PostgresPracticeRepository {
    #[shaku(inject)]
    pub db_connection: Arc<dyn DatabaseConnection>,
}

#[derive(Debug, Clone, Default)]
pub struct PracticeFilter {
    pub ids: Option<Vec<Uuid>>,
}

#[async_trait]
pub trait PracticeRepository: Interface {
    async fn find_many(&self, filter: PracticeFilter) -> Result<Vec<Practice>, AppError>;

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError>;
    async fn save(&self, practice: Practice) -> Result<Practice, AppError>;
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl PracticeRepository for PostgresPracticeRepository {
    async fn find_many(&self, filter: PracticeFilter) -> Result<Vec<Practice>, AppError> {
        let mut query = Query::select().expr(Expr::cust("*")).from(Practices::Table).to_owned();

        if let Some(ids) = &filter.ids {
            query.and_where(Expr::col(Practices::Id).is_in(ids.clone()));
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let practices = sqlx_query::<Postgres, Practice, _>(&sql, values)
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(practices)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError> {
        let (sql, values) = Query::select()
            .expr(Expr::cust("*"))
            .from(Practices::Table)
            .and_where(Expr::col(Practices::Id).eq(*id))
            .build_sqlx(PostgresQueryBuilder);

        let practice = sqlx_query::<Postgres, Practice, _>(&sql, values)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(practice)
    }

    async fn save(&self, practice: Practice) -> Result<Practice, AppError> {
        let query = r#"
            INSERT INTO practices (id, enterprise_name,location, description, supervisor_name, supervisor_email, supervisor_phone, start_date, end_date, practice_status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET 
                enterprise_name = EXCLUDED.enterprise_name,
                location = EXCLUDED.location,
                description = EXCLUDED.description,
                supervisor_name = EXCLUDED.supervisor_name,
                supervisor_email = EXCLUDED.supervisor_email,
                supervisor_phone = EXCLUDED.supervisor_phone,
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                practice_status = EXCLUDED.practice_status
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, Practice>(query)
            .bind(practice.id)
            .bind(practice.enterprise_name)
            .bind(practice.location)
            .bind(practice.description)
            .bind(practice.supervisor_name)
            .bind(practice.supervisor_email)
            .bind(practice.supervisor_phone)
            .bind(practice.start_date)
            .bind(practice.end_date)
            .bind(practice.practice_status)
            .fetch_one(self.db_connection.get_pool())
            .await?;
        Ok(result)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        let (sql, values) = Query::delete()
            .from_table(Practices::Table)
            .and_where(Expr::col(Practices::Id).eq(*id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with::<Postgres, _>(&sql, values)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
