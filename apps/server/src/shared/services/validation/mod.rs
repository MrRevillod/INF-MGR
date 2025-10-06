use std::{collections::HashMap, sync::LazyLock};

use sword::prelude::*;

pub struct FileValidationService;

static VALIDATION_FUNCTIONS: LazyLock<HashMap<&'static str, fn(&[u8]) -> bool>> =
    LazyLock::new(|| {
        HashMap::from([
            ("pdf", infer::archive::is_pdf as fn(&[u8]) -> bool),
            ("zip", infer::archive::is_zip as fn(&[u8]) -> bool),
        ])
    });

pub struct FileValidationConfig {
    pub kind: &'static str,
    pub name: &'static str,
}

impl MiddlewareWithConfig<FileValidationConfig> for FileValidationService {
    async fn handle(
        config: FileValidationConfig,
        mut ctx: Context,
        next: Next,
    ) -> MiddlewareResult {
        let FileValidationConfig { kind, name } = config;

        let mut multipart = ctx.multipart().await?;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            eprintln!("Error reading multipart field: {}", e);
            HttpResponse::InternalServerError()
                .message(format!("Failed to read multipart field: {name}"))
        })? {
            let Some(field_name) = field.name() else {
                continue;
            };

            if field_name == name {
                let data = field.bytes().await.map_err(|e| {
                    eprintln!("Error reading field bytes: {}", e);
                    HttpResponse::InternalServerError()
                        .message(format!("Failed to read bytes for field: {name}"))
                })?;

                if !VALIDATION_FUNCTIONS[kind](&data) {
                    return Err(HttpResponse::UnsupportedMediaType().message(
                        "The file type isn't valid for the requested resource",
                    ));
                }

                ctx.extensions.insert(data.clone());
                break;
            }
        }

        next!(ctx, next)
    }
}
