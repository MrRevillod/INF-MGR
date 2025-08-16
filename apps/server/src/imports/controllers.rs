use sword::prelude::*;

#[controller("/imports")]
pub struct ImportsController {}

#[routes]
impl ImportsController {
    #[post("/course")]
    async fn import_couse(ctx: Context) -> HttpResult<HttpResponse> {
        todo!()
    }
}
