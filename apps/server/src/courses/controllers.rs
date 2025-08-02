use sword::prelude::*;
use uuid::Uuid;

use crate::{
    courses::{CourseResponse, CourseService, CreateCourseDto, UpdateCourseDto},
    shared::di::AppModule,
};

#[controller("/courses")]
pub struct CoursesController {}

#[routes]
impl CoursesController {
    #[get("/")]
    async fn get_courses(ctx: Context) -> HttpResult<HttpResponse> {
        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;
        let asignatures = service
            .get_all()
            .await?
            .into_iter()
            .map(CourseResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(asignatures))
    }

    #[post("/")]
    async fn create_course(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateCourseDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;

        let asignature = service.create(input).await?;

        Ok(HttpResponse::Created().data(asignature))
    }

    #[patch("/{id}")]
    async fn update_course(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateCourseDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;
        let updated_asignature = service.update(&asignature_id, input).await?;

        Ok(HttpResponse::Ok().data(updated_asignature))
    }

    #[delete("/{id}")]
    async fn delete_course(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let service = ctx.get_dependency::<AppModule, dyn CourseService>()?;

        service.delete(&asignature_id).await?;

        Ok(HttpResponse::Ok())
    }
}
