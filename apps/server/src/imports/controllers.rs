use sword::prelude::*;

use crate::{imports::ImportService, shared::AppModule};

#[controller("/imports")]
pub struct ImportsController {}

#[routes]
impl ImportsController {
    #[post("/course")]
    async fn import_couse(ctx: Context) -> HttpResult<HttpResponse> {
        let service = ctx.di::<AppModule, dyn ImportService>()?;

        service.import_course_students().await?;

        Ok(HttpResponse::Created())
    }
}
