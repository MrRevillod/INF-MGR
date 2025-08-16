use sword::prelude::*;

use crate::{
    container::AppModule,
    imports::{ImportCourseDto, ImportService},
};

#[controller("/imports")]
pub struct ImportsController {}

#[routes]
impl ImportsController {
    #[post("/course")]
    async fn import_couse(ctx: Context) -> HttpResult<HttpResponse> {
        let data = ctx.validated_body::<ImportCourseDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn ImportService>()?;

        service.import_course_students(data).await?;

        Ok(HttpResponse::Created())
    }
}
