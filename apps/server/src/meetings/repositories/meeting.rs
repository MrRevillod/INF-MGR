use sword::core::injectable;

use crate::{
    meetings::*,
    shared::{AppResult, PostgresDatabase},
    types::*,
};

#[injectable]
pub struct MeetingRepository {
    db_connection: Arc<PostgresDatabase>,
}

impl MeetingRepository {
    pub async fn find_many(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>> {
        let mut query = QueryBuilder::new("SELECT * FROM meetings WHERE 1=1");

        if let Some(attendee_id) = filter.attendee_id {
            query.push(" AND ");
            query.push_bind(attendee_id);
            query.push(" = ANY(attendees)");
        }

        let meetings = query
            .build_query_as::<Meeting>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(meetings)
    }

    pub async fn save(&self, meeting: &Meeting) -> AppResult<Meeting> {
        let result = sqlx::query_as::<_, Meeting>("SELECT * FROM save_meeting($1)")
            .bind(meeting.to_json()?)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }
}
