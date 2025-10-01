use crate::{
    meetings::*,
    shared::{AppResult, DatabaseConnection},
    types::*,
};

#[derive(Component)]
#[shaku(interface = MeetingRequestsRepository)]
pub struct PostgresMeetingRequestsRepository {
    #[shaku(inject)]
    #[allow(unused)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
pub trait MeetingRequestsRepository: Interface {
    async fn find_many(
        &self,
        filter: MeetingRequestFilter,
    ) -> AppResult<Vec<MeetingRequest>>;

    async fn create(&self, meeting_req: MeetingRequest)
    -> AppResult<MeetingRequest>;
}

#[async_trait]
impl MeetingRequestsRepository for PostgresMeetingRequestsRepository {
    async fn find_many(
        &self,
        f: MeetingRequestFilter,
    ) -> AppResult<Vec<MeetingRequest>> {
        let mut query =
            QueryBuilder::new("SELECT * FROM meeting_requests WHERE 1=1");

        if let Some(student_id) = f.student_id {
            query.push(" AND $1 = ANY(attendees)");
            query.push_bind(student_id);
        }

        if let Some(teacher_id) = f.teacher_id {
            query.push(" AND $1 = ANY(attendees)");
            query.push_bind(teacher_id);
        }

        let meeting_reqs = query
            .build_query_as::<MeetingRequest>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(meeting_reqs)
    }

    async fn create(
        &self,
        meeting_req: MeetingRequest,
    ) -> AppResult<MeetingRequest> {
        let mut query = QueryBuilder::new(
            "INSERT INTO meeting_requests (id, status, attendees, course_id, created_at, updated_at) ",
        );

        query.push("VALUES ($1, $2, $3, $4, $5, $6) RETURNING *");
        query.push_bind(meeting_req.id);
        query.push_bind(meeting_req.status.to_string());
        query.push_bind(meeting_req.attendees);
        query.push_bind(meeting_req.course_id);
        query.push_bind(meeting_req.created_at);
        query.push_bind(meeting_req.updated_at);

        let created_meeting_req = query
            .build_query_as::<MeetingRequest>()
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(created_meeting_req)
    }
}
