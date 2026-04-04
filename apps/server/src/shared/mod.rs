pub mod errors;
pub use errors::*;
pub mod event_queue;
pub mod http;
pub mod macros;

mod injectables;
pub use injectables::*;

mod validation;
pub use validation::*;

pub mod utils {
    use super::errors::{AppError, AppResult};
    use serde::Serialize;
    use serde_json::{Value, to_value};

    pub trait ToJson {
        fn to_json(&self) -> AppResult<Value>;
    }

    impl<T> ToJson for T
    where
        T: Serialize,
    {
        fn to_json(&self) -> AppResult<Value> {
            to_value(self).map_err(|e| AppError::InternalServerError(e.into()))
        }
    }
}
