use std::{fmt::Debug, ops::Deref};

use axum::extract::{FromRequest, Json, Query, Request};
use axum_responses::http::HttpResponse;

use serde_json::{json, Map, Value};
use validator::{Validate, ValidationErrors};

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

        data.validate().map_err(|errors| {
            HttpResponse::build().code(400).body(json!({
                "message": "Invalid request body",
                "errors": map_validation_errors(&errors)
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

        value.validate().map_err(|errors| {
            HttpResponse::build().code(400).body(json!({
                "error": "Invalid Query format",
                "errors": map_validation_errors(&errors)
            }))
        })?;

        Ok(Self(value))
    }
}

pub fn map_validation_errors(e: &ValidationErrors) -> Value {
    let mut map = Map::new();

    for (field, errs) in e.field_errors() {
        let msgs = errs
            .iter()
            .map(|err| {
                err.message
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| err.code.to_string())
            })
            .map(Value::String)
            .collect();

        map.insert(field.to_string(), Value::Array(msgs));
    }

    Value::Object(map)
}
