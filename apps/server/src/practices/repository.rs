use sword::core::injectable;

use crate::{
    practices::entity::Practice,
    shared::{AppResult, PostgresDatabase},
    types::*,
};

#[injectable]
pub struct PracticeRepository {
    pub db_connection: Arc<PostgresDatabase>,
}

#[derive(Debug, Clone, Default)]
pub struct PracticeFilter {
    pub ids: Option<Vec<Uuid>>,
}

impl PracticeRepository {
    pub async fn find_many(&self, filter: PracticeFilter) -> AppResult<Vec<Practice>> {
        let mut query = QueryBuilder::new("SELECT * FROM practices WHERE 1=1");

        if let Some(ids) = &filter.ids
            && !ids.is_empty()
        {
            query.push(" AND id = ANY(");
            query.push_bind(ids);
            query.push(")");
        }

        let practices = query
            .build_query_as::<Practice>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(practices)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Practice>> {
        let r = sqlx::query_as::<_, Practice>("SELECT * FROM practices WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(r)
    }

    pub async fn save(&self, practice: Practice) -> AppResult<Practice> {
        let query = r"
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
        ";

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

    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM practices WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
