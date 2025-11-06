use infer::archive::{is_pdf, is_zip};
use std::{collections::HashMap, sync::LazyLock};
use sword::prelude::*;

#[middleware]
pub struct FileValidationService;

type ValidationFunction = fn(&[u8]) -> bool;
pub type RequestFiles = HashMap<&'static str, Vec<u8>>;

static VALIDATION_FUNCTIONS: LazyLock<HashMap<&'static str, ValidationFunction>> =
    LazyLock::new(|| {
        HashMap::from([
            ("pdf", is_pdf as ValidationFunction),
            ("zip", is_zip as ValidationFunction),
        ])
    });

pub struct FileValidationConfig {
    pub kind: &'static str,
    pub name: &'static str,
}

impl OnRequestWithConfig<FileValidationConfig> for FileValidationService {
    async fn on_request_with_config(
        &self,
        config: FileValidationConfig,
        mut req: Request,
    ) -> MiddlewareResult {
        let FileValidationConfig { kind, name } = config;

        let mut multipart = req.multipart().await?;

        if req.extensions.get::<RequestFiles>().is_none() {
            req.extensions.insert::<RequestFiles>(HashMap::new());
        }

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            eprintln!("Error reading multipart field: {e}");
            HttpResponse::InternalServerError()
                .message(format!("Failed to read multipart field: {name}"))
        })? {
            let Some(field_name) = field.name() else {
                continue;
            };

            if field_name == name {
                let data = field.bytes().await.map_err(|e| {
                    eprintln!("Error reading field bytes: {e}");
                    HttpResponse::InternalServerError()
                        .message(format!("Failed to read bytes for field: {name}"))
                })?;

                if !VALIDATION_FUNCTIONS[kind](&data) {
                    return Err(HttpResponse::UnsupportedMediaType().message(
                        "The file type isn't valid for the requested resource",
                    ));
                }

                let req_files = req
                    .extensions
                    .get_mut::<RequestFiles>()
                    .expect("RequestFiles should be initialized");

                req_files.insert(name, data.to_vec());

                break;
            }
        }

        req.next().await
    }
}
