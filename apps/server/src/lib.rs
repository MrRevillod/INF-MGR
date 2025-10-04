pub mod auth;
pub mod config;
pub mod courses;
pub mod enrollments;
pub mod imports;
pub mod meetings;
pub mod practices;
pub mod users;

pub mod shared;

pub mod types {

    pub type QueryBuilder<'args> = SqlxQueryBuilder<'args, Postgres>;
    pub use async_trait::async_trait;
    pub use chrono::{DateTime, Duration, Utc};
    pub use serde::{Deserialize, Serialize};
    pub use shaku::{Component, Interface};
    pub use sqlx::{FromRow, Postgres, QueryBuilder as SqlxQueryBuilder, Type};
    pub use std::fmt::Display;
    pub use std::sync::Arc;
    pub use thiserror::Error;
    pub use uuid::Uuid;
    pub use validator::Validate;

    pub use sea_query::{
        Expr, Iden, Order, PostgresQueryBuilder, Query, extension::postgres::PgExpr,
    };

    pub use sea_query_sqlx::SqlxBinder;

    pub use google_calendar3::api::{Event, EventAttendee, EventDateTime};
}
