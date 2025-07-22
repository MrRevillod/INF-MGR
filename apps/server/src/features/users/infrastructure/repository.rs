use async_trait::async_trait;
use shaku::Component;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::database::DatabaseConnection;

use crate::users::infrastructure::Role;
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
    async fn find_all(&self, role: String) -> Result<Vec<User>, UserError> {
        let pool = self.database_connection.get_pool();

        let role = Role::from_str(&role).unwrap_or(Role::Student);

        let users = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users WHERE roles @> ARRAY[$1]::user_role[]",
        )
        .bind(role)
        .fetch_all(pool)
        .await?;

        let entity_vec = users.into_iter().map(User::from).collect();

        Ok(entity_vec)
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE id = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        Ok(user.map(User::from))
    }

    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE rut = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(rut)
            .fetch_optional(pool)
            .await?;

        Ok(user.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"SELECT * FROM users WHERE email = $1"#;

        let user = sqlx::query_as::<_, UserModel>(query)
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user.map(User::from))
    }

    async fn create(&self, user: User) -> Result<User, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"
            INSERT INTO users (id, rut, name, email, password, roles) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, rut, name, email, password, roles
        "#;

        let roles = user
            .roles
            .iter()
            .map(|r| Role::from_str(r).unwrap_or(Role::Student))
            .collect::<Vec<_>>();

        let model = sqlx::query_as::<_, UserModel>(query)
            .bind(user.id)
            .bind(user.rut)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(roles)
            .fetch_one(pool)
            .await?;

        Ok(User::from(model))
    }

    async fn update(&self, user: User) -> Result<User, UserError> {
        let pool = self.database_connection.get_pool();
        let query = r#"
            UPDATE users 
            SET email = $1, password = $2, roles = $3
            WHERE id = $4
        "#;

        let roles = user
            .roles
            .iter()
            .map(|r| Role::from_str(r).unwrap_or(Role::Student))
            .collect::<Vec<_>>();

        sqlx::query(query)
            .bind(&user.email)
            .bind(&user.password)
            .bind(roles)
            .bind(user.id)
            .execute(pool)
            .await?;

        Ok(user)
    }

    async fn delete(&self, user_id: &Uuid) -> Result<(), UserError> {
        let pool = self.database_connection.get_pool();

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
