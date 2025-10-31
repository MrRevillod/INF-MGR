use std::sync::Arc;

use sword::prelude::*;
use uuid::Uuid;

use crate::imports::ImportService;

#[controller("/imports")]
pub struct ImportsController {
    import_service: Arc<ImportService>,
}

#[routes]
impl ImportsController {
    #[post("/course/{course_id}/students")]
    async fn import_course(&self, req: Request) -> HttpResult {
        let course_id = req.param::<Uuid>("course_id")?;

        self.import_service
            .import_course_students(&course_id)
            .await?;

        Ok(HttpResponse::Created())
    }
}
