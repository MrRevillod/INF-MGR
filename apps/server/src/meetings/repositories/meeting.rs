use crate::{
    meetings::*,
    shared::{AppResult, DatabaseConnection},
    types::*,
};

#[derive(Component)]
#[shaku(interface = MeetingRepository)]
pub struct PostgresMeetingRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
pub trait MeetingRepository: Interface {
    async fn find_many(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>>;
}

#[async_trait]
impl MeetingRepository for PostgresMeetingRepository {
    async fn find_many(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>> {
        let mut query = QueryBuilder::new("SELECT * FROM meetings WHERE 1=1");

        if let Some(attendee_id) = filter.attendee_id {
            query.push(" AND $1 = ANY(attendees)");
            query.push_bind(attendee_id);
        }

        let meetings = query
            .build_query_as::<Meeting>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        return Ok(meetings);
    }
}
