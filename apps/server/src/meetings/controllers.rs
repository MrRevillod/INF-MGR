use sword::prelude::*;

use crate::{auth::*, meetings::*, shared::http::ContextExt, types::*, users::Role};

#[controller("/meetings")]
#[uses(Authentication)]
pub struct MeetingsController {
    meetings: Arc<MeetingService>,
    meeting_requests: Arc<MeetingRequestService>,
}

#[routes]
impl MeetingsController {
    #[get("/")]
    #[uses(MinimumRequiredRole, config = Role::Student)]
    #[doc = "Listar todas las reuniones programadas"]
    #[doc = "Recibe un query opcional para filtrar las reuniones por attendee_id"]
    async fn list_meetings(&self, req: Request) -> HttpResult {
        let mut query = req
            .query_validator::<GetMeetingsQueryDto>()?
            .unwrap_or_default();

        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            match owner_validation.user_role {
                Role::Student => {
                    query.attendee_id = Some(owner_validation.user_id.to_string())
                }
                Role::Teacher => {
                    query.attendee_id = Some(owner_validation.user_id.to_string())
                }
                _ => {}
            }
        }

        let meetings = self.meetings.get_all(query.into()).await?;

        Ok(HttpResponse::Ok().data(meetings))
    }

    #[get("/schedule/{meeting_id}")]
    #[uses(MinimumRequiredRole, config = Role::Teacher)]
    #[doc = "Programar una reunión a partir de una solicitud de reunión existente"]
    async fn schedule_meeting(&self, req: Request) -> HttpResult {
        let user = req.get_current_user()?;
        let meeting_id = req.param::<Uuid>("meeting_id")?;

        let OwnershipValidation { required, .. } = req.get_ownership_validation()?;

        if !self
            .meeting_requests
            .can_schedule(&user, &meeting_id)
            .await?
            && required
        {
            return Err(HttpResponse::Unauthorized()
                .message("No tienes permiso para programar esta reunión."));
        }

        let input = req.body_validator::<ScheduleMeetingDto>()?;
        let meeting = self.meetings.schedule(&meeting_id, input).await?;

        Ok(HttpResponse::Created().data(meeting))
    }

    #[get("/requests")]
    #[uses(MinimumRequiredRole, config = Role::Student)]
    #[doc = "Listar todas las solicitudes de reunión"]
    async fn list_meeting_requests(&self, req: Request) -> HttpResult {
        let query = req
            .query_validator::<GetMeetingRequestsQueryDto>()?
            .unwrap_or_default();

        let mut filter = MeetingRequestFilter::from(query);

        let owner_validation = req.get_ownership_validation()?;

        if owner_validation.required {
            match owner_validation.user_role {
                Role::Student => filter.student_id = Some(owner_validation.user_id),
                Role::Teacher => filter.teacher_id = Some(owner_validation.user_id),
                _ => {}
            }
        }

        let meeting_reqs = self.meeting_requests.get_all(filter).await?;

        Ok(HttpResponse::Ok().data(meeting_reqs))
    }

    #[post("/request")]
    #[uses(MinimumRequiredRole, config = Role::Student)]
    #[doc = "Crear una nueva solicitud de reunión"]
    async fn create_meeting_request(&self, req: Request) -> HttpResult {
        let body = req.body_validator::<CreateMeetingRequestDto>()?;
        let meeting_req = self.meeting_requests.create(body).await?;

        Ok(HttpResponse::Created().data(meeting_req))
    }
}
