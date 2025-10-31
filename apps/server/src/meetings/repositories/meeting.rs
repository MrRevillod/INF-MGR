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
            query.push(" AND $1 = ANY(attendees)");
            query.push_bind(attendee_id);
        }

        let meetings = query
            .build_query_as::<Meeting>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(meetings)
    }

    pub async fn save(&self, meeting: &Meeting) -> AppResult<Meeting> {
        let saved_meeting = sqlx::query_as::<_, Meeting>(
            "INSERT INTO meetings (id, google_event_id, summary, description, start_date, end_date, attendees, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                google_event_id = EXCLUDED.google_event_id,
                summary = EXCLUDED.summary,
                description = EXCLUDED.description,
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                attendees = EXCLUDED.attendees,
                updated_at = EXCLUDED.updated_at
            RETURNING *",
        )
        .bind(meeting.id)
        .bind(&meeting.google_event_id)
        .bind(&meeting.summary)
        .bind(&meeting.description)
        .bind(meeting.start_date)
        .bind(meeting.end_date)
        .bind(&meeting.attendees)
        .bind(meeting.created_at)
        .bind(meeting.updated_at)
        .fetch_one(self.db_connection.get_pool())
        .await?;

        Ok(saved_meeting)
    }
}
