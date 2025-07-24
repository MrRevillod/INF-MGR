use sword::web::HttpResponse;

use crate::reports::domain::ReportError;

impl From<ReportError> for HttpResponse {
    fn from(value: ReportError) -> Self {
        match value {
            ReportError::NotFound => HttpResponse::NotFound(),
            ReportError::Database { source } => {
                eprintln!("Report entity Database error: {source}");
                HttpResponse::InternalServerError()
            }
        }
    }
}
