pub mod errors;
pub use errors::{AppError, AppResult, AuthError, NotFoundError, ValidationError};
pub mod macros;

pub mod infrastructure {
    pub mod di {
        mod builder;
        mod container;

        pub use builder::DependencyContainer;
        pub use container::{AppModule, InitialComponents};
    }

    mod databases {
        mod postgres;
        mod redis;

        pub use postgres::*;
        pub use redis::*;
    }

    pub mod http;
    pub mod layers;
    pub mod oauth;
    pub mod uuid;

    pub use databases::*;
    pub use di::*;
    pub use http::*;
    pub use layers::*;
    pub use oauth::*;
    pub use uuid::*;
}

pub use infrastructure::*;

pub mod services {
    pub mod errors;
    pub mod mailer;
    pub mod printer;
    pub mod templates {
        mod context;
        mod files;

        pub use context::*;
        pub use files::*;
    }

    pub mod event_queue {
        pub mod publisher;
        pub mod subscriber;

        pub use publisher::*;
        pub use subscriber::*;
    }

    mod validation;
    pub use validation::PdfValidationService;

    mod calendar {
        mod hub;
        pub use hub::*;
    }

    pub use calendar::*;

    pub use crate::template_ctx;
    pub use errors::*;
    pub use event_queue::*;
    pub use mailer::*;
    pub use printer::*;
    pub use templates::*;

    use chrono::{DateTime, Utc};
    use chrono_tz::America::Santiago;

    pub fn format_date(date: String) -> String {
        let date = DateTime::parse_from_rfc3339(&date)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();

        date.map(|date| date.with_timezone(&Santiago).format("%d/%m/%y").to_string())
            .unwrap_or_default()
    }
}
