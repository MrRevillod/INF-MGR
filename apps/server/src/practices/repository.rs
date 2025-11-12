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
        let result = sqlx::query_as::<_, Practice>("SELECT * FROM save_practice($1)")
            .bind(practice.to_json()?)
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
