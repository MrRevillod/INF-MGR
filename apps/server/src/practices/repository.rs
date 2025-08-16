use std::sync::Arc;

use async_trait::async_trait;
use sea_query::{Expr, ExprTrait, OnConflict, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{query_as_with as sqlx_query, Postgres};
use uuid::Uuid;

use crate::{
    practices::entity::{Practice, Practices, PRACTICE_INSERT_COLUMNS, PRACTICE_UPDATE_COLUMNS},
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
        let (sql, values) = Query::insert()
            .into_table(Practices::Table)
            .columns(PRACTICE_INSERT_COLUMNS)
            // SAFETY: values_panic is safe here because we control the input data
            // and ensure proper types. Consider refactoring to use values() with proper error handling
            .values_panic(vec![
                practice.id.into(),
                practice.enterprise_name.into(),
                practice.location.into(),
                practice.description.into(),
                practice.supervisor_name.into(),
                practice.supervisor_email.into(),
                practice.supervisor_phone.into(),
                practice.start_date.into(),
                practice.end_date.into(),
                practice.is_approved.into(),
            ])
            .on_conflict(
                OnConflict::columns([Practices::Id])
                    .update_columns(PRACTICE_UPDATE_COLUMNS)
                    .to_owned(),
            )
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let saved_practice = sqlx_query::<Postgres, Practice, _>(&sql, values)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(saved_practice)
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
