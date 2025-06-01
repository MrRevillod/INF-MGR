use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::infrastructure::DatabaseConnection;

use crate::features::user::{
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
    async fn find_all(&self) -> Result<Vec<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let users = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users"#)
            .fetch_all(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        let entity_vec = users.into_iter().map(|model| User::from(model)).collect();

        Ok(entity_vec)
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE id = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        Ok(user.map(|model| User::from(model)))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE email = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        Ok(user.map(|model| User::from(model)))
    }

    async fn find_by_username(&self, name: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE username = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(name)
            .fetch_optional(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        Ok(user.map(|model| User::from(model)))
    }

    async fn create(&self, user: User) -> Result<User, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"
            INSERT INTO users (
                id, username, email, password, validated, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            RETURNING id, username, email, password, validated, created_at, updated_at
        "#;

        let model = sqlx::query_as::<_, UserModel>(query)
            .bind(user.id)
            .bind(user.username)
            .bind(user.email)
            .bind(user.password)
            .bind(user.validated)
            .bind(user.created_at)
            .bind(user.updated_at)
            .fetch_one(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        Ok(User::from(model))
    }

    async fn update(&self, user: User) -> Result<User, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"
            UPDATE users 
            SET username = $1, email = $2, password = $3, updated_at = $4
            WHERE id = $5
        "#;

        sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.updated_at)
            .bind(user.id)
            .execute(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        Ok(user)
    }

    async fn delete(&self, user_id: Uuid) -> Result<(), UserError> {
        let pool = self.database_connection.get_pool();

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|_| UserError::UnexpectedError)?;

        Ok(())
    }
}
