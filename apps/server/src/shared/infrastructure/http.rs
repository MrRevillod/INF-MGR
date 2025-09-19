use sword::__internal::IntoResponse;
use sword::web::{Context, HttpResponse, StatusCode};

use crate::auth::OwnershipValidation;

pub trait ContextExt {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse>;
}

impl ContextExt for Context {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse> {
        let ownership_validation = self.extensions.get::<OwnershipValidation>();

        let Some(value) = ownership_validation else {
            return Err(HttpResponse::InternalServerError()
                .message("OwnershipValidation not found in context"));
        };

        Ok(value.clone())
    }
}

pub enum FileResponse {
    Document(Vec<u8>),
}

impl IntoResponse for FileResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            FileResponse::Document(data) => {
                (StatusCode::OK, [("Content-type", "application/pdf")], data)
                    .into_response()
            }
        }
    }
}
