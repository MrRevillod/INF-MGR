use std::sync::Arc;

use sword::prelude::*;
use uuid::Uuid;

use crate::imports::{ImportService, ImportStudentsRequest};

#[controller("/imports")]
pub struct ImportsController {
    import_service: Arc<ImportService>,
}

#[routes]
impl ImportsController {
    #[post("/course/{course_id}/students")]
    async fn import_course(&self, req: Request) -> HttpResult {
        let course_id = req.param::<Uuid>("course_id")?;
        let input = req.body_validator::<ImportStudentsRequest>()?;

        self.import_service
            .import_course_students(&course_id, input.students)
            .await?;

        Ok(HttpResponse::Created())
    }
}
