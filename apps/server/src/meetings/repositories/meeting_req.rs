use sword::core::injectable;

use crate::{
    meetings::*,
    shared::{AppResult, PostgresDatabase},
    types::*,
};

#[injectable]
pub struct MeetingRequestsRepository {
    db_connection: Arc<PostgresDatabase>,
}

impl MeetingRequestsRepository {
    pub async fn find_many(
        &self,
        f: MeetingRequestFilter,
    ) -> AppResult<Vec<MeetingRequest>> {
        let mut query = QueryBuilder::new("SELECT * FROM meeting_requests WHERE 1=1");

        if let Some(student_id) = f.student_id {
            query.push(" AND ");
            query.push_bind(student_id);
            query.push(" = ANY(attendees)");
        }

        if let Some(teacher_id) = f.teacher_id {
            query.push(" AND ");
            query.push_bind(teacher_id);
            query.push(" = ANY(attendees)");
        }
        let meeting_reqs = query
            .build_query_as::<MeetingRequest>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(meeting_reqs)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<MeetingRequest>> {
        let meeting_req = sqlx::query_as::<_, MeetingRequest>(
            "SELECT * FROM meeting_requests WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.db_connection.get_pool())
        .await?;

        Ok(meeting_req)
    }

    pub async fn save(&self, meeting_req: MeetingRequest) -> AppResult<MeetingRequest> {
        let result =
            sqlx::query_as::<_, MeetingRequest>("SELECT * FROM save_meeting_request($1)")
                .bind(meeting_req.to_json()?)
                .fetch_one(self.db_connection.get_pool())
                .await?;

        Ok(result)
    }
}
