use std::sync::Arc;

use async_trait::async_trait;
use sea_query::{Expr, ExprTrait, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{Postgres, query_as_with as sqlx_query};
use uuid::Uuid;

use crate::{
    meetings::{Meeting, MeetingFilter, Meetings},
    shared::{AppResult, DatabaseConnection},
};

#[derive(Component)]
#[shaku(interface = MeetingRepository)]
pub struct MeetingRepositoryImpl {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
pub trait MeetingRepository: Interface {
    async fn save(&self, meeting: Meeting) -> AppResult<Meeting>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Meeting>>;
    async fn find_many(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>>;
}

#[async_trait]
impl MeetingRepository for MeetingRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Meeting>> {
        let meeting =
            sqlx::query_as::<_, Meeting>("SELECT * FROM meetings WHERE id = $1")
                .bind(id)
                .fetch_optional(self.db_connection.get_pool())
                .await?;

        Ok(meeting)
    }

    async fn find_many(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>> {
        let mut query = Query::select()
            .expr(Expr::cust("*"))
            .from(Meetings::Table)
            .to_owned();

        if let Some(teacher_id) = filter.teacher_id {
            query.and_where(Expr::col(Meetings::TeacherId).eq(teacher_id));
        }

        if let Some(student_id) = filter.student_id {
            query.and_where(Expr::col(Meetings::StudentId).eq(student_id));
        }

        if let Some(status) = filter.status {
            query.and_where(Expr::col(Meetings::Status).eq(status.to_string()));
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let result = sqlx_query::<Postgres, Meeting, _>(&sql, values)
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    async fn save(&self, meeting: Meeting) -> AppResult<Meeting> {
        let query = r#"
            INSERT INTO meetings (id, teacher_id, student_id, start_date, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                teacher_id = EXCLUDED.teacher_id,
                student_id = EXCLUDED.student_id,
                start_date = EXCLUDED.start_date,
                status = EXCLUDED.status,
                created_at = EXCLUDED.created_at
            RETURNING *;
        "#;

        let saved_meeting = sqlx::query_as::<_, Meeting>(query)
            .bind(meeting.id)
            .bind(meeting.teacher_id)
            .bind(meeting.student_id)
            .bind(meeting.start_date)
            .bind(meeting.status)
            .bind(meeting.created_at)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(saved_meeting)
    }
}
