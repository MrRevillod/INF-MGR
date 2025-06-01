pub mod interfaces {
    mod create;
    mod delete;
    mod get;
    mod update;

    pub use create::*;
    pub use delete::*;
    pub use get::*;
    pub use update::*;
}

pub mod services {
    mod password;
    pub use password::*;
}

pub mod usecases {
    mod create;
    mod delete;
    mod get;
    mod update;

    pub use create::*;
    pub use delete::*;
    pub use get::*;
    pub use update::*;
}
