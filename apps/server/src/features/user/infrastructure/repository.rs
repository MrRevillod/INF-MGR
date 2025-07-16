use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

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
        let users = sqlx::query_as::<_, UserModel>("SELECT * FROM users")
            .fetch_all(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        let entity_vec = users.into_iter().map(|model| User::from(model)).collect();

        Ok(entity_vec)
    }

    async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE id = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        Ok(user.map(|model| User::from(model)))
    }

    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE rut = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(rut)
            .fetch_optional(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        Ok(user.map(|model| User::from(model)))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE email = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        Ok(user.map(|model| User::from(model)))
    }

    async fn create(&self, user: User) -> Result<User, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"
            INSERT INTO users (id, rut, name, email, password, role) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, rut, name, email, password, role
        "#;

        let model = sqlx::query_as::<_, UserModel>(query)
            .bind(user.id)
            .bind(user.rut)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.role)
            .fetch_one(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        Ok(User::from(model))
    }

    async fn update(&self, user: User) -> Result<User, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"
            UPDATE users 
            SET email = $1, password = $2, role = $3
            WHERE id = $4
        "#;

        sqlx::query(query)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.id)
            .execute(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        Ok(user)
    }

    async fn delete(&self, user_id: &str) -> Result<(), UserError> {
        let pool = self.database_connection.get_pool();

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| UserError::UnexpectedError(e.to_string()))?;

        Ok(())
    }
}
