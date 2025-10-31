use std::sync::Arc;
use sword::core::injectable;
use uuid::Uuid;

use crate::{
    shared::{AppResult, DEFAULT_PAGE_SIZE, PostgresDatabase},
    types::*,
    users::User,
};

#[injectable]
pub struct UserRepository {
    database_connection: Arc<PostgresDatabase>,
}

#[derive(Default, Clone)]
pub struct UserFilter {
    pub search: Option<String>,
    pub page: u64,
    pub id: Option<Uuid>,
    pub rut: Option<String>,
    pub email: Option<String>,
    pub ids: Option<Vec<Uuid>>,
    pub ruts: Option<Vec<String>>,
}

impl UserRepository {
    pub async fn find_many(&self, filter: UserFilter) -> AppResult<Vec<User>> {
        let mut query = QueryBuilder::new("SELECT * FROM users WHERE 1=1");

        if let Some(ids) = &filter.ids
            && !ids.is_empty()
        {
            query.push(" AND id = ANY(");
            query.push_bind(ids);
            query.push(")");
        }

        if let Some(ruts) = &filter.ruts
            && !ruts.is_empty()
        {
            query.push(" AND rut = ANY(");
            query.push_bind(ruts);
            query.push(")");
        }

        if let Some(search) = filter.search {
            let pattern = format!("%{search}%");
            query.push(" AND (name ILIKE ");
            query.push_bind(pattern.clone());
            query.push(" OR email ILIKE ");
            query.push_bind(pattern.clone());
            query.push(" OR rut ILIKE ");
            query.push_bind(pattern);
            query.push(")");
        }

        query.push(" ORDER BY created_at DESC ");
        query.push(" LIMIT ").push_bind(DEFAULT_PAGE_SIZE as i64);
        query
            .push(" OFFSET ")
            .push_bind((filter.page.saturating_sub(1) * DEFAULT_PAGE_SIZE) as i64);

        let results = query
            .build_query_as::<User>()
            .fetch_all(self.database_connection.get_pool())
            .await?;

        Ok(results)
    }

    pub async fn find_one(&self, filter: UserFilter) -> AppResult<Option<User>> {
        let mut query = QueryBuilder::new("SELECT * FROM users WHERE 1=1");

        if let Some(id) = filter.id {
            query.push(" AND id = ").push_bind(id);
        }

        if let Some(rut) = filter.rut {
            query.push(" AND rut = ").push_bind(rut);
        }

        if let Some(email) = filter.email {
            query.push(" AND email = ").push_bind(email);
        }

        let user = query
            .build_query_as::<User>()
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, user_id: &Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    pub async fn save(&self, user: User) -> AppResult<User> {
        let upsert_query = r"
            INSERT INTO users (id, rut, name, email, google_id, role, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) 
            DO UPDATE SET 
                rut = EXCLUDED.rut,
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                google_id = EXCLUDED.google_id,
                role = EXCLUDED.role
            WHERE users.deleted_at IS NULL
            RETURNING *
        ";

        let saved_user = sqlx::query_as::<_, User>(upsert_query)
            .bind(user.id)
            .bind(user.rut)
            .bind(user.name)
            .bind(user.email)
            .bind(user.google_id)
            .bind(user.role)
            .bind(user.created_at)
            .bind(user.deleted_at)
            .fetch_one(self.database_connection.get_pool())
            .await?;

        Ok(saved_user)
    }

    pub async fn create_many(&self, users: Vec<User>) -> AppResult<Vec<User>> {
        if users.is_empty() {
            return Ok(vec![]);
        }

        let mut query_values = Vec::new();
        let mut arg_index = 1;

        for _ in &users {
            query_values.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${}, ${}, ${})",
                arg_index,     // id
                arg_index + 1, // rut
                arg_index + 2, // name
                arg_index + 3, // email
                arg_index + 4, // google_id
                arg_index + 5, // role
                arg_index + 6, // created_at
                arg_index + 7, // deleted_at
            ));

            arg_index += 8;
        }

        let query = format!(
            r" 
                INSERT INTO users (id, rut, name, email, google_id, role, created_at, deleted_at)
                VALUES {}
                ON CONFLICT (id) DO NOTHING
                RETURNING *
            ",
            query_values.join(", ")
        );

        let mut sqlx_query = sqlx::query_as::<_, User>(&query);

        for user in &users {
            sqlx_query = sqlx_query
                .bind(user.id)
                .bind(&user.rut)
                .bind(&user.name)
                .bind(&user.email)
                .bind(&user.google_id)
                .bind(&user.role)
                .bind(user.created_at)
                .bind(user.deleted_at);
        }

        let results = sqlx_query
            .fetch_all(self.database_connection.get_pool())
            .await?;

        Ok(results)
    }

    pub async fn delete(&self, user_id: &Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(self.database_connection.get_pool())
            .await?;

        Ok(())
    }

    pub async fn count(&self, filter: UserFilter) -> AppResult<i64> {
        let mut query = QueryBuilder::new("SELECT COUNT(id) FROM users WHERE 1=1");

        if let Some(search) = filter.search {
            let pattern = format!("%{search}%");
            query.push(" AND (name ILIKE ");
            query.push_bind(pattern.clone());
            query.push(" OR email ILIKE ");
            query.push_bind(pattern.clone());
            query.push(" OR rut ILIKE ");
            query.push_bind(pattern);
            query.push(")");
        }

        let count: (i64,) = query
            .build_query_as()
            .fetch_one(self.database_connection.get_pool())
            .await?;

        Ok(count.0)
    }
}
