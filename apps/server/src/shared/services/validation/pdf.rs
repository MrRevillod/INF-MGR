use sword::prelude::*;

pub struct PdfValidationService;

impl MiddlewareWithConfig<&str> for PdfValidationService {
    async fn handle(name: &str, mut ctx: Context, next: Next) -> MiddlewareResult {
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

                if !infer::archive::is_pdf(&data) {
                    return Err(HttpResponse::BadRequest().message(format!(
                        "Uploaded file is not a valid PDF for field: {name}"
                    )))?;
                }

                ctx.extensions.insert(data.clone());
                break;
            }
        }

        next!(ctx, next)
    }
}
