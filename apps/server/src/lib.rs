pub mod config;

pub mod users {
    pub mod domain;

    pub mod application {
        mod interfaces {
            mod create;
            mod delete;
            mod get;
            mod update;

            pub use create::{CreateUserCase, CreateUserInput};
            pub use delete::DeleteUserCase;
            pub use get::GetUsersCase;
            pub use update::{UpdateUserCase, UpdateUserInput};
        }

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            pub use create::CreateUserCaseImpl;
            pub use delete::DeleteUserCaseImpl;
            pub use get::GetUsersCaseImpl;
            pub use update::UpdateUserCaseImpl;
        }

        pub use interfaces::*;
        pub use usecases::*;
    }

    pub mod infrastructure {
        mod controllers;
        mod dtos;
        mod models;
        mod repository;

        pub mod errors;
        pub use controllers::UserController;
        pub use dtos::{CreateUserDto, UpdateUserDto, UserResponseDTO};
        pub use models::{Role, UserModel};
        pub use repository::PostgresUserRepository;
    }
}

pub mod asignatures {
    pub mod domain;

    pub mod application {
        mod interfaces {
            mod create;
            mod delete;
            mod get;
            mod update;

            // pub use create::{CreateUserCase, CreateUserInput};
            // pub use delete::DeleteUserCase;
            // pub use get::GetUsersCase;
            // pub use update::{UpdateUserCase, UpdateUserInput};
        }

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            // pub use create::CreateUserCaseImpl;
            // pub use delete::DeleteUserCaseImpl;
            // pub use get::GetUsersCaseImpl;
            // pub use update::UpdateUserCaseImpl;
        }

        pub use interfaces::*;
        pub use usecases::*;
    }

    pub mod infrastructure {
        mod controllers;
        mod dtos;
        mod models;
        mod repository;

        pub mod errors;
        // pub use controllers::UserController;
        // pub use dtos::{CreateUserDto, UpdateUserDto, UserResponseDTO};
        // pub use models::{Role, UserModel};
        // pub use repository::PostgresUserRepository;
    }
}

pub mod reports {
    pub mod domain;

    pub mod application {
        mod interfaces {
            mod create;
            mod delete;
            mod get;
            mod update;

            // pub use create::{CreateUserCase, CreateUserInput};
            // pub use delete::DeleteUserCase;
            // pub use get::GetUsersCase;
            // pub use update::{UpdateUserCase, UpdateUserInput};
        }

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            // pub use create::CreateUserCaseImpl;
            // pub use delete::DeleteUserCaseImpl;
            // pub use get::GetUsersCaseImpl;
            // pub use update::UpdateUserCaseImpl;
        }

        pub use interfaces::*;
        pub use usecases::*;
    }

    pub mod infrastructure {
        mod controllers;
        mod dtos;
        mod models;
        mod repository;

        pub mod errors;
        // pub use controllers::UserController;
        // pub use dtos::{CreateUserDto, UpdateUserDto, UserResponseDTO};
        // pub use models::{Role, UserModel};
        // pub use repository::PostgresUserRepository;
    }
}

pub mod students {
    pub mod domain;

    pub mod application {
        mod interfaces {
            mod create;
            mod delete;
            mod get;
            mod update;

            // pub use create::{CreateUserCase, CreateUserInput};
            // pub use delete::DeleteUserCase;
            // pub use get::GetUsersCase;
            // pub use update::{UpdateUserCase, UpdateUserInput};
        }

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            // pub use create::CreateUserCaseImpl;
            // pub use delete::DeleteUserCaseImpl;
            // pub use get::GetUsersCaseImpl;
            // pub use update::UpdateUserCaseImpl;
        }

        pub use interfaces::*;
        pub use usecases::*;
    }

    pub mod infrastructure {
        mod controllers;
        mod dtos;
        mod models;
        mod repository;

        pub mod errors;
        // pub use controllers::UserController;
        // pub use dtos::{CreateUserDto, UpdateUserDto, UserResponseDTO};
        // pub use models::{Role, UserModel};
        // pub use repository::PostgresUserRepository;
    }
}

pub mod practices {
    pub mod domain;

    pub mod application {
        mod interfaces {
            mod create;
            mod delete;
            mod get;
            mod update;

            // pub use create::{CreateUserCase, CreateUserInput};
            // pub use delete::DeleteUserCase;
            // pub use get::GetUsersCase;
            // pub use update::{UpdateUserCase, UpdateUserInput};
        }

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            // pub use create::CreateUserCaseImpl;
            // pub use delete::DeleteUserCaseImpl;
            // pub use get::GetUsersCaseImpl;
            // pub use update::UpdateUserCaseImpl;
        }

        pub use interfaces::*;
        pub use usecases::*;
    }

    pub mod infrastructure {
        mod controllers;
        mod dtos;
        mod models;
        mod repository;

        pub mod errors;
        // pub use controllers::UserController;
        // pub use dtos::{CreateUserDto, UpdateUserDto, UserResponseDTO};
        // pub use models::{Role, UserModel};
        // pub use repository::PostgresUserRepository;
    }
}

pub mod shared {
    pub mod services {
        pub mod errors;
        mod password;
        pub use password::{BcryptPasswordHasher, PasswordHasher};
    }

    pub mod database;
    pub mod di;
    pub mod layers;
}
