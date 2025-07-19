use sword::prelude::*;
use uuid::Uuid;

use crate::inscriptions::{
    application::{
        CreateInscriptionCase, DeleteInscriptionCase, GetInscriptionsCase,
        UpdateInscriptionCase,
    },
    infrastructure::{
        dtos::InscriptionQueryDto, CreateInscriptionDto, InscriptionModel,
        UpdateInscriptionDto,
    },
};

use crate::shared::di::AppModule;

#[controller("/inscriptions")]
pub struct InscriptionController {}

#[routes]
impl InscriptionController {
    #[get("/")]
    async fn find_all(ctx: Context) -> HttpResult<HttpResponse> {
        let filter = ctx.validated_query_optional::<InscriptionQueryDto>()?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetInscriptionsCase>()?;

        let inscriptions =
            use_case.execute(filter.unwrap_or_default().into()).await?;

        let models: Vec<InscriptionModel> =
            inscriptions.into_iter().map(Into::into).collect();

        Ok(HttpResponse::Ok().data(models))
    }

    #[post("/")]
    async fn create(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateInscriptionDto>()?;
        let use_case =
            ctx.get_dependency::<AppModule, dyn CreateInscriptionCase>()?;

        let inscription = use_case.execute(input.into()).await?;

        Ok(HttpResponse::Created().data(InscriptionModel::from(inscription)))
    }

    #[patch("/{id}")]
    async fn update(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateInscriptionDto>()?;

        let use_case =
            ctx.get_dependency::<AppModule, dyn UpdateInscriptionCase>()?;

        let updated = use_case.execute(&id, input.into()).await?;

        Ok(HttpResponse::Ok().data(InscriptionModel::from(updated)))
    }

    #[delete("/{id}")]
    async fn delete(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let use_case =
            ctx.get_dependency::<AppModule, dyn DeleteInscriptionCase>()?;

        use_case.execute(&id).await?;

        Ok(HttpResponse::Ok())
    }
}
