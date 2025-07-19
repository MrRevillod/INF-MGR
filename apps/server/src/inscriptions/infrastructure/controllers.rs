use sword::prelude::*;

use crate::{
    inscriptions::{
        application::GetInscriptionsCase,
        infrastructure::{dtos::InscriptionQueryDto, InscriptionModel},
    },
    shared::di::AppModule,
};

#[controller("/inscriptions")]
pub struct InscriptionController {}

#[routes]
impl InscriptionController {
    #[get("/")]
    async fn find_all(ctx: Context) -> HttpResult<HttpResponse> {
        let filter = ctx.validated_query::<InscriptionQueryDto>()?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetInscriptionsCase>()?;

        let inscriptions = use_case.execute(filter.into()).await?;
        let models: Vec<InscriptionModel> =
            inscriptions.into_iter().map(Into::into).collect();

        Ok(HttpResponse::Ok().data(models))
    }

    #[post("/")]
    async fn create(ctx: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }

    #[patch("/{id}")]
    async fn update(ctx: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }

    #[delete("/{id}")]
    async fn delete(ctx: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok())
    }
}
