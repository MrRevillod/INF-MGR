pub mod auth;
pub mod config;
pub mod courses;
pub mod enrollments;
pub mod imports;
pub mod logger;
pub mod meetings;
pub mod practices;
pub mod shared;
pub mod users;

pub mod types {
    pub type QueryBuilder<'args> = SqlxQueryBuilder<'args, Postgres>;
    pub use super::shared::utils::ToJson;
    pub use chrono::{DateTime, Duration, Utc};
    pub use google_calendar3::api::{Event, EventAttendee, EventDateTime};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{Value, json};
    pub use sqlx::{FromRow, Postgres, QueryBuilder as SqlxQueryBuilder, Type};
    pub use std::fmt::Display;
    pub use std::sync::Arc;
    pub use thiserror::Error;
    pub use uuid::Uuid;
    pub use validator::Validate;
}

pub use services::*;
