use async_trait::async_trait;
use chrono::Utc;
use shaku::Component;
use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::database::DatabaseConnection;

use crate::users::domain::FindAllReturnType;
use crate::users::{
    domain::{GetUsersParams, User, UserError, UserRepository},
    infrastructure::models::{vec_string_to_roles, UserModel},
};

const PAGE_SIZE: usize = 10;

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    database_connection: Arc<dyn DatabaseConnection>,
}

impl PostgresUserRepository {
    fn apply_user_filters<'a>(
        builder: &mut QueryBuilder<'a, Postgres>,
        filter: &GetUsersParams,
    ) {
        if let Some(search) = &filter.search {
            let pattern = format!("%{}%", search);

            builder.push(" AND (");
            builder
                .push("name ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR email ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR rut ILIKE ")
                .push_bind(pattern.clone());
            builder.push(")");
        }

        let roles = vec_string_to_roles(filter.roles.clone());

        if !roles.is_empty() {
            builder.push(" AND roles && ");
            builder.push_bind(roles);
            builder.push("::user_role[]");
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_all(
        &self,
        filter: GetUsersParams,
    ) -> Result<FindAllReturnType, UserError> {
        // Primero obtener el total de registros
        let mut count_query = QueryBuilder::<Postgres>::new(
            "SELECT COUNT(*) FROM users WHERE deleted_at IS NULL",
        );
        Self::apply_user_filters(&mut count_query, &filter);

        let total_users: i64 = count_query
            .build_query_scalar()
            .fetch_one(self.database_connection.get_pool())
            .await?;

        // Luego obtener los datos paginados
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT * FROM users WHERE deleted_at IS NULL",
        );

        Self::apply_user_filters(&mut query, &filter);

        query.push(" ORDER BY created_at DESC");

        let offset = (filter.page - 1) * PAGE_SIZE;

        query.push(" LIMIT ");
        query.push_bind(PAGE_SIZE as i64);

        query.push(" OFFSET ");
        query.push_bind(offset as i64);

        let all_users = query
            .build_query_as::<UserModel>()
            .fetch_all(self.database_connection.get_pool())
            .await?
            .into_iter()
            .map(User::from)
            .collect::<Vec<User>>();

        let total_pages = (total_users as f64 / PAGE_SIZE as f64).ceil() as usize;
        let has_next = filter.page < total_pages;
        let has_previous = filter.page > 1;

        Ok(FindAllReturnType {
            users: all_users,
            current_page: filter.page,
            total_pages,
            total_users: total_users as usize,
            has_next,
            has_previous,
        })
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
            INSERT INTO users (id, rut, name, email, password, roles, deleted_at, created_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, rut, name, email, password, roles, deleted_at, created_at
        "#;

        let model = sqlx::query_as::<_, UserModel>(query)
            .bind(user.id)
            .bind(user.rut)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(vec_string_to_roles(user.roles))
            .bind(user.deleted_at)
            .bind(user.created_at)
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
