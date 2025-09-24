use serde::Deserialize;
use sqlx::{Pool, Postgres};
use sword::prelude::*;
use validator::Validate;

use crate::{COURSE_CODE_REGEX, RequireApiKey, models::Course};

#[controller("/")]
pub struct AppController {}

#[derive(Debug, Deserialize, Validate)]
pub struct CourseParams {
    #[validate(range(min = 2025, max = 2100))]
    pub year: i32,

    #[validate(regex(path = *COURSE_CODE_REGEX))]
    pub code: String,
}

#[routes]
impl AppController {
    #[get("/courses/{year}/{code}/students")]
    #[middleware(RequireApiKey)]
    async fn get_students(ctx: Context) -> HttpResult<HttpResponse> {
        let params = ctx.validated_params::<CourseParams>()?;
        let db_pool = ctx.get_state::<Pool<Postgres>>()?;

        let students = Course::get_students(&db_pool, params.year, &params.code)
            .await
            .map_err(|e| {
                eprintln!("Database query error: {:?}", e);
                HttpResponse::InternalServerError()
            })?;

        Ok(HttpResponse::Ok().data(students))
    }
}
