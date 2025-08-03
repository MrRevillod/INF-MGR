use async_trait::async_trait;
use chrono::Utc;
use shaku::{Component, Interface};
use sqlx::{FromRow, Postgres, QueryBuilder};
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::{database::DatabaseConnection, entities::Pagination};
use crate::users::User;

pub const DEFAULT_PAGE_SIZE: usize = 10;

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    database_connection: Arc<dyn DatabaseConnection>,
}

#[derive(Default)]
pub struct UserFilter {
    pub search: Option<String>,
    pub page: i64,
}

#[derive(FromRow, Debug)]
pub struct UserWithCount {
    #[sqlx(flatten)]
    pub user: User,
    pub total_count: i64,
}

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_all(
        &self,
        filter: UserFilter,
    ) -> Result<Pagination<User>, AppError>;

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    async fn save(&self, user: User) -> Result<User, AppError>;
    async fn delete(&self, user_id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_all(
        &self,
        filter: UserFilter,
    ) -> Result<Pagination<User>, AppError> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT *, COUNT(*) OVER() as total_count FROM users 
            WHERE deleted_at IS NULL",
        );

        if let Some(search) = &filter.search {
            let pattern = format!("%{search}%");

            query.push(" AND (");
            query
                .push("name ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR email ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR rut ILIKE ")
                .push_bind(pattern.clone());
            query.push(")");
        }

        query.push(" ORDER BY created_at DESC");
        query.push(" LIMIT ");
        query.push_bind(DEFAULT_PAGE_SIZE as i64);
        query.push(" OFFSET ");
        query.push_bind((filter.page - 1) * DEFAULT_PAGE_SIZE as i64);

        let results = query
            .build_query_as::<UserWithCount>()
            .fetch_all(self.database_connection.get_pool())
            .await?;

        let total_users =
            results.first().map(|r| r.total_count).unwrap_or(0) as usize;

        let all_users = results.into_iter().map(|r| r.user).collect::<Vec<User>>();

        let total_pages =
            (total_users as f64 / DEFAULT_PAGE_SIZE as f64).ceil() as i64;

        Ok(Pagination::<User> {
            items: all_users,
            current_page: filter.page as usize,
            total_pages: total_pages as usize,
            has_next: filter.page < total_pages as i64,
            has_previous: filter.page > 1,
        })
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, AppError> {
        let query = "SELECT * FROM users WHERE id = $1";

        let user = sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE rut = $1")
            .bind(rut)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let query = r#"SELECT * FROM users WHERE email = $1"#;

        let user = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    async fn save(&self, user: User) -> Result<User, AppError> {
        let upsert_query = r#"
            INSERT INTO users (id, rut, name, email, password, roles, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NULL)
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
            .fetch_one(self.database_connection.get_pool())
            .await?;

        Ok(saved_user)
    }

    async fn delete(&self, user_id: &Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET deleted_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(user_id)
            .execute(self.database_connection.get_pool())
            .await?;

        Ok(())
    }
}
