use sword::web::HttpResponse;

use crate::practices::domain::PracticeError;

impl From<PracticeError> for HttpResponse {
    fn from(error: PracticeError) -> Self {
        match error {
            PracticeError::NotFound => HttpResponse::NotFound(),
            PracticeError::Database { source } => {
                eprintln!("Database error: {source}");
                HttpResponse::InternalServerError()
            }
        }
    }
}
