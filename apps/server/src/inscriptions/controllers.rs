use sword::prelude::*;
use uuid::Uuid;

use crate::inscriptions::{
    CreateInscriptionDto, GetInscriptionsDto, InscriptionResponse,
    InscriptionService, UpdateInscriptionDto,
};

use crate::shared::di::AppModule;

#[controller("/inscriptions")]
pub struct InscriptionsController {}

#[routes]
impl InscriptionsController {
    #[get("/")]
    async fn find_all(ctx: Context) -> HttpResult<HttpResponse> {
        let filter = ctx.validated_query_optional::<GetInscriptionsDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn InscriptionService>()?;

        let result = service
            .get_all(filter.unwrap_or_default().into())
            .await?
            .into_iter()
            .map(InscriptionResponse::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(result))
    }

    #[post("/")]
    async fn create(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateInscriptionDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn InscriptionService>()?;

        let inscription = service.create(input).await?;

        Ok(HttpResponse::Created().data(inscription))
    }

    #[patch("/{id}")]
    async fn update(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateInscriptionDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn InscriptionService>()?;
        let updated = service.update(&id, input).await?;

        Ok(HttpResponse::Ok().data(updated))
    }

    #[delete("/{id}")]
    async fn remove(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let service = ctx.get_dependency::<AppModule, dyn InscriptionService>()?;

        service.remove(&id).await?;

        Ok(HttpResponse::Ok())
    }
}
