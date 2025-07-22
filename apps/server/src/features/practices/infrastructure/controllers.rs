use sword::prelude::*;
use uuid::Uuid;

use crate::{
    practices::{
        application::{
            CreatePracticeCase, DeletePracticeCase, GetPracticeCase,
            UpdatePracticeCase,
        },
        infrastructure::{
            dtos::{CreatePracticeDto, UpdatePracticeDto},
            models::PracticeModel,
        },
    },
    shared::di::AppModule,
};

#[controller("/practices")]
pub struct PracticesController {}

#[routes]
impl PracticesController {
    #[get("/{id}")]
    async fn get_by_id(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetPracticeCase>()?;

        let practice = use_case.execute(&id).await?;

        Ok(HttpResponse::Ok().data(PracticeModel::from(practice)))
    }

    #[post("/")]
    async fn create(ctx: Context) -> HttpResult<HttpResponse> {
        let body = ctx.validated_body::<CreatePracticeDto>()?;
        let use_case = ctx.get_dependency::<AppModule, dyn CreatePracticeCase>()?;

        let practice = use_case.execute(body.into()).await?;

        Ok(HttpResponse::Created().data(PracticeModel::from(practice)))
    }

    #[put("/{id}")]
    async fn update(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let body = ctx.validated_body::<UpdatePracticeDto>()?;

        let use_case = ctx.get_dependency::<AppModule, dyn UpdatePracticeCase>()?;
        let practice = use_case.execute(&id, body.into()).await?;

        Ok(HttpResponse::Ok().data(PracticeModel::from(practice)))
    }

    #[delete("/{id}")]
    async fn delete(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let use_case = ctx.get_dependency::<AppModule, dyn DeletePracticeCase>()?;

        use_case.execute(&id).await?;

        Ok(HttpResponse::Ok())
    }
}
