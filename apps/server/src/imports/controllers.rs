use sword::prelude::*;
use uuid::Uuid;

use crate::{config::StudentsApiConfig, imports::ImportService, shared::AppModule};

#[controller("/imports")]
pub struct ImportsController {}

#[routes]
impl ImportsController {
    #[post("/course/{course_id}/students")]
    async fn import_course(ctx: Context) -> HttpResult<HttpResponse> {
        let course_id = ctx.param::<Uuid>("course_id")?;
        let api_conf = ctx.config::<StudentsApiConfig>()?;

        ctx.di::<AppModule, dyn ImportService>()?
            .import_course_students(api_conf, &course_id)
            .await?;

        Ok(HttpResponse::Created())
    }
}
