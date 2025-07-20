use sword::prelude::*;
use uuid::Uuid;

use crate::{
    asignatures::{
        application::{
            CreateAsignatureCase, DeleteAsignatureCase, GetAsignaturesCase,
            UpdateAsignatureCase,
        },
        infrastructure::{
            AsignatureModel, CreateAsignatureDto, UpdateAsignatureDto,
        },
    },
    shared::di::AppModule,
};

#[controller("/asignatures")]
pub struct AsignaturesController {}

#[routes]
impl AsignaturesController {
    #[get("/")]
    async fn get_asignatures(ctx: Context) -> HttpResult<HttpResponse> {
        let use_case = ctx.get_dependency::<AppModule, dyn GetAsignaturesCase>()?;
        let asignatures = use_case.execute().await?;

        let models: Vec<AsignatureModel> =
            asignatures.into_iter().map(AsignatureModel::from).collect();

        Ok(HttpResponse::Ok().data(models))
    }

    #[post("/")]
    async fn create_asignature(ctx: Context) -> HttpResult<HttpResponse> {
        let input = ctx.validated_body::<CreateAsignatureDto>()?;
        let case = ctx.get_dependency::<AppModule, dyn CreateAsignatureCase>()?;

        let asignature = case.execute(input.try_into()?).await?;
        let model = AsignatureModel::from(asignature);

        Ok(HttpResponse::Created().data(model))
    }

    #[patch("/{id}")]
    async fn update_asignature(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let input = ctx.validated_body::<UpdateAsignatureDto>()?;

        let case = ctx.get_dependency::<AppModule, dyn UpdateAsignatureCase>()?;

        let updated_asignature = case.execute(&asignature_id, input.into()).await?;
        let model = AsignatureModel::from(updated_asignature);

        Ok(HttpResponse::Ok().data(model))
    }

    #[delete("/{id}")]
    async fn delete_asignature(ctx: Context) -> HttpResult<HttpResponse> {
        let asignature_id = ctx.param::<Uuid>("id")?;
        let case = ctx.get_dependency::<AppModule, dyn DeleteAsignatureCase>()?;

        case.execute(&asignature_id).await?;

        Ok(HttpResponse::Ok())
    }
}
