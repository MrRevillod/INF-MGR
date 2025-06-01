use std::ops::Deref;

use axum::extract::{FromRequest, Json, Query, Request};

use axum_responses::http::HttpResponse;
use serde_json::json;
use validator::Validate;

pub struct BodyValidator<T>(pub T);

impl<S, T> FromRequest<S> for BodyValidator<T>
where
    T: Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = HttpResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) =
            Json::<T>::from_request(req, state).await.map_err(|_| {
                HttpResponse::build()
                    .code(400)
                    .body(json!({ "error": "Invalid request body" }))
            })?;

        data.validate().map_err(|e| {
            HttpResponse::build().code(400).body(json!({
                "message": "Invalid request body",
                "errors": e.to_string()
            }))
        })?;

        Ok(BodyValidator(data))
    }
}

#[derive(Debug)]
pub struct QueryValidator<T>(pub T);

impl<T> Deref for QueryValidator<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
impl<S, T> FromRequest<S> for QueryValidator<T>
where
    S: Send + Sync,
    T: Validate + for<'de> serde::Deserialize<'de> + Send,
{
    type Rejection = HttpResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) =
            Query::<T>::from_request(req, state).await.map_err(|_| {
                HttpResponse::build()
                    .code(400)
                    .body(json!({ "error": "Invalid Query format" }))
            })?;

        value.validate().map_err(|err| {
            HttpResponse::build().code(400).body(json!({
                "error": "Invalid Query format",
                "errors": err.to_string()
            }))
        })?;

        Ok(Self(value))
    }
}
