use sword::prelude::*;

use crate::{
    auth::{Authentication, MinimumRequiredRole},
    meetings::*,
    shared::{AppModule, ContextExt},
    users::Role,
};

#[controller("/meetings")]
pub struct MeetingsController {}

#[routes]
impl MeetingsController {
    #[get("/")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "student")]
    async fn list_meetings(ctx: Context) -> HttpResult<HttpResponse> {
        let query = ctx
            .validated_query::<GetMeetingsQueryDto>()?
            .unwrap_or_default();

        let filter = MeetingFilter::from(query);

        let meetings = ctx
            .di::<AppModule, dyn MeetingService>()?
            .get_all(filter)
            .await?;

        Ok(HttpResponse::Ok().data(meetings))
    }

    #[get("/schedule/{meeting_id}")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    async fn schedule_meeting(_: Context) -> HttpResult<HttpResponse> {
        todo!()
    }

    #[get("/requests")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "student")]
    async fn list_meeting_requests(ctx: Context) -> HttpResult<HttpResponse> {
        let query = ctx
            .validated_query::<GetMeetingRequestsQueryDto>()?
            .unwrap_or_default();

        let mut filter = MeetingRequestFilter::from(query);

        let owner_validation = ctx.get_ownership_validation()?;

        if owner_validation.required {
            match owner_validation.user_role {
                Role::Student => filter.student_id = Some(owner_validation.user_id),
                Role::Teacher => filter.teacher_id = Some(owner_validation.user_id),
                _ => {}
            }
        }

        let meeting_reqs = ctx
            .di::<AppModule, dyn MeetingRequestService>()?
            .get_all(filter)
            .await?;

        Ok(HttpResponse::Ok().data(meeting_reqs))
    }

    #[post("/request")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "student")]
    async fn create_meeting_request(ctx: Context) -> HttpResult<HttpResponse> {
        let body = ctx.validated_body::<CreateMeetingRequestDto>()?;

        let meeting_req = ctx
            .di::<AppModule, dyn MeetingRequestService>()?
            .create(body)
            .await?;

        Ok(HttpResponse::Created().data(meeting_req))
    }
}
