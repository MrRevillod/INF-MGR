use sword::prelude::*;

use crate::{
    auth::{Authentication, MinimumRequiredRole},
    meetings::{CreateMeetingDto, MeetingService},
    shared::AppModule,
};

#[controller("/meetings")]
pub struct MeetingsController {}

#[routes]
impl MeetingsController {
    #[post("/request")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "student")]
    async fn create_meeting_request(ctx: Context) -> HttpResult<HttpResponse> {
        let dto = ctx.validated_body::<CreateMeetingDto>()?;

        let meeting = ctx
            .di::<AppModule, dyn MeetingService>()?
            .create(dto)
            .await?;

        Ok(HttpResponse::Created().data(meeting))
    }

    #[post("/schedule")]
    #[middleware(Authentication)]
    #[middleware(MinimumRequiredRole, config = "teacher")]
    async fn schedule_meeting(ctx: Context) -> HttpResult<HttpResponse> {
        todo!()
    }
}
