use async_trait::async_trait;
use chrono::Utc;
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{query_as_with as sqlx_query, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use sea_query::{
    extension::postgres::PgExpr, Expr, ExprTrait, Order, PostgresQueryBuilder, Query,
};

use crate::{
    shared::{
        database::DatabaseConnection, entities::DEFAULT_PAGE_SIZE, errors::AppError,
    },
    users::entity::{User, Users},
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    database_connection: Arc<dyn DatabaseConnection>,
}

#[derive(Default, Clone)]
pub struct UserFilter {
    pub search: Option<String>,
    pub page: u64,
    pub id: Option<Uuid>,
    pub rut: Option<String>,
    pub email: Option<String>,
    pub ids: Option<Vec<Uuid>>,
}

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_many(&self, filter: UserFilter) -> Result<Vec<User>, AppError>;
    async fn find_one(&self, filter: UserFilter) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, AppError>;

    async fn save(&self, user: User) -> Result<User, AppError>;
    async fn delete(&self, user_id: &Uuid) -> Result<(), AppError>;
    async fn count(&self, filter: UserFilter) -> Result<i64, AppError>;
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_many(&self, filter: UserFilter) -> Result<Vec<User>, AppError> {
        let mut query = Query::select()
            .expr(Expr::cust("*"))
            .from(Users::Table)
            .to_owned();

        if let Some(ids) = &filter.ids {
            query.and_where(Expr::col(Users::Id).is_in(ids.clone()));
        }

        if let Some(search) = &filter.search {
            let pattern = format!("%{search}%");

            query.and_where(
                Expr::col(Users::Name)
                    .ilike(pattern.clone())
                    .or(Expr::col(Users::Email).ilike(pattern.clone()))
                    .or(Expr::col(Users::Rut).ilike(pattern.clone())),
            );
        }

        query.order_by(Users::CreatedAt, Order::Desc);
        query.limit(DEFAULT_PAGE_SIZE);
        // Avoid underflow when page == 0
        query.offset(filter.page.saturating_sub(1) * DEFAULT_PAGE_SIZE);

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let results = sqlx_query::<Postgres, User, _>(&sql, values)
            .fetch_all(self.database_connection.get_pool())
            .await?;

        Ok(results)
    }

    async fn find_one(&self, filter: UserFilter) -> Result<Option<User>, AppError> {
        let (sql, values) = Query::select()
            .expr(Expr::cust("*"))
            .from(Users::Table)
            .apply_if(filter.id, |q, value| {
                q.and_where(Expr::col(Users::Id).eq(value));
            })
            .apply_if(filter.rut, |q, value| {
                q.and_where(Expr::col(Users::Rut).eq(value));
            })
            .apply_if(filter.email, |q, value| {
                q.and_where(Expr::col(Users::Email).eq(value));
            })
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx_query::<Postgres, User, _>(&sql, values)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, AppError> {
        let (sql, values) = Query::select()
            .expr(Expr::cust("*"))
            .from(Users::Table)
            .and_where(Expr::col(Users::Id).eq(*user_id))
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx_query::<Postgres, User, _>(&sql, values)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    async fn save(&self, user: User) -> Result<User, AppError> {
        let upsert_query = r#"
            INSERT INTO users (id, rut, name, email, password, roles, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) 
            DO UPDATE SET 
                rut = EXCLUDED.rut,
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                password = EXCLUDED.password,
                roles = EXCLUDED.roles
            WHERE users.deleted_at IS NULL
            RETURNING *
        "#;

        let saved_user = sqlx::query_as::<_, User>(upsert_query)
            .bind(user.id)
            .bind(user.rut)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.roles)
            .bind(user.created_at)
            .bind(user.deleted_at)
            .fetch_one(self.database_connection.get_pool())
            .await?;

        Ok(saved_user)
    }

    async fn delete(&self, user_id: &Uuid) -> Result<(), AppError> {
        let (sql, values) = Query::update()
            .table(Users::Table)
            .value(Users::DeletedAt, Utc::now())
            .and_where(Expr::col(Users::Id).eq(*user_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.database_connection.get_pool())
            .await?;

        Ok(())
    }

    async fn count(&self, filter: UserFilter) -> Result<i64, AppError> {
        let mut query = Query::select()
            .expr(Expr::count(Expr::col(Users::Id)))
            .from(Users::Table)
            .to_owned();

        if let Some(search) = filter.search {
            let search_pattern = format!("%{search}%");
            query = query
                .and_where(
                    Expr::col(Users::Name)
                        .ilike(&search_pattern)
                        .or(Expr::col(Users::Email).ilike(&search_pattern))
                        .or(Expr::col(Users::Rut).ilike(&search_pattern)),
                )
                .to_owned();
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let count: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(self.database_connection.get_pool())
            .await?;

        Ok(count.0)
    }
}
