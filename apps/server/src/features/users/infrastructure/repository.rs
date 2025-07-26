use async_trait::async_trait;
use chrono::Utc;
use shaku::Component;
use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::database::DatabaseConnection;

use crate::users::domain::GetUsersParams;
use crate::users::infrastructure::models::vec_string_to_roles;
use crate::users::{
    domain::{User, UserError, UserRepository},
    infrastructure::models::UserModel,
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    database_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_all(
        &self,
        filter: GetUsersParams,
    ) -> Result<Vec<User>, UserError> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT * FROM users WHERE deleted_at IS NULL",
        );

        if let Some(search) = filter.search {
            let pattern = format!("%{}%", search);

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

        let roles = vec_string_to_roles(filter.roles);

        if !roles.is_empty() {
            query.push(" AND roles && ");
            query.push_bind(roles);
            query.push("::user_role[]");
        }

        let users = query
            .build_query_as::<UserModel>()
            .fetch_all(self.database_connection.get_pool())
            .await?;

        Ok(users.into_iter().map(User::from).collect())
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserError> {
        let query = "SELECT * FROM users WHERE id = $1";

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(user_id)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user.map(User::from))
    }

    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, UserError> {
        let user =
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE rut = $1")
                .bind(rut)
                .fetch_optional(self.database_connection.get_pool())
                .await?;

        Ok(user.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let query = r#"SELECT * FROM users WHERE email = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(email)
            .fetch_optional(self.database_connection.get_pool())
            .await?;

        Ok(user.map(User::from))
    }

    async fn create(&self, user: User) -> Result<User, UserError> {
        let query = r#"
            INSERT INTO users (id, rut, name, email, password, roles, deleted_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, rut, name, email, password, roles, deleted_at
        "#;

        let model = sqlx::query_as::<_, UserModel>(query)
            .bind(user.id)
            .bind(user.rut)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(vec_string_to_roles(user.roles))
            .bind(user.deleted_at)
            .fetch_one(self.database_connection.get_pool())
            .await?;

        Ok(User::from(model))
    }

    async fn update(&self, user: User) -> Result<User, UserError> {
        let query = r#"
            UPDATE users 
            SET email = $1, password = $2, roles = $3
            WHERE id = $4
        "#;

        sqlx::query(query)
            .bind(&user.email)
            .bind(&user.password)
            .bind(vec_string_to_roles(user.roles.clone()))
            .bind(user.id)
            .execute(self.database_connection.get_pool())
            .await?;

        Ok(user)
    }

    async fn delete(&self, user_id: &Uuid) -> Result<(), UserError> {
        sqlx::query("UPDATE users SET deleted_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(user_id)
            .execute(self.database_connection.get_pool())
            .await?;

        Ok(())
    }
}
