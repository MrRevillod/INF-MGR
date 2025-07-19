pub mod config;

pub mod users {
    pub mod domain;

    #[cfg(test)]
    pub mod __tests__;

    pub mod application {
        mod interfaces;

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

    #[cfg(test)]
    pub mod __tests__;

    pub mod domain;

    pub mod application {
        mod interfaces;

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            pub use create::CreateAsignatureCaseImpl;
            pub use delete::DeleteAsignatureCaseImpl;
            pub use get::GetAsignaturesCaseImpl;
            pub use update::UpdateAsignatureCaseImpl;
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
        pub use controllers::AsignaturesController;
        pub use dtos::{CreateAsignatureDto, UpdateAsignatureDto};
        pub use models::{AsignatureModel, EvaluationType};
        pub use repository::PostgresAsignatureRepository;
    }
}

pub mod reports {
    pub mod domain;

    pub mod application {
        mod interfaces;

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            // pub use create::;
            // pub use delete::;
            // pub use get::;
            // pub use update::;
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

pub mod inscriptions {
    pub mod domain;

    #[cfg(test)]
    pub mod __tests__;

    pub mod application {
        mod interfaces;

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            pub use create::CreateInscriptionCaseImpl;
            pub use delete::DeleteInscriptionCaseImpl;
            pub use get::GetInscriptionsCaseImpl;
            pub use update::UpdateInscriptionCaseImpl;
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
        pub use controllers::InscriptionController;
        pub use dtos::{
            CreateInscriptionDto, StudentEvaluationDto, UpdateInscriptionDto,
        };
        pub use models::{InscriptionModel, StudentEvaluationModel};
        pub use repository::PostgresInscriptionRepository;
    }
}

pub mod practices {
    pub mod domain;

    pub mod application {
        mod interfaces;

        mod usecases {
            pub mod create;
            pub mod delete;
            pub mod get;
            pub mod update;

            // pub use create::;
            // pub use delete::;
            // pub use get::;
            // pub use update::;
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
        mod mailer;
        mod password;
        pub use mailer::{MailContext, MailTo, Mailer, MailerService};
        pub use password::{BcryptPasswordHasher, PasswordHasher};
    }

    pub mod database;
    pub mod di;
    pub mod layers;
    pub mod smtp;
    pub mod validators;
}

#[cfg(test)]
pub mod tests {

    use axum_test::TestServer;
    use sword::prelude::Application;

    use super::{
        asignatures::infrastructure::AsignaturesController,
        config::{MailerConfig, PostgresDbConfig},
        shared::{
            database::PostgresDatabase, di::DependencyContainer,
            smtp::LettreTransport,
        },
        users::infrastructure::UserController,
    };

    pub async fn init_test_app() -> TestServer {
        let app = Application::builder().expect("Failed to build Application");

        let pg_db_config = app.config.get::<PostgresDbConfig>().unwrap();
        let mailer_config = app.config.get::<MailerConfig>().unwrap();

        let postgres_db = PostgresDatabase::new(&pg_db_config)
            .await
            .expect("Failed to create database connection");

        postgres_db
            .migrate()
            .await
            .expect("Failed to migrate database");

        sqlx::query("TRUNCATE users, asignatures, students, practices CASCADE")
            .execute(&postgres_db.pool)
            .await
            .expect("Failed to truncate database tables");

        let smtp_transport = LettreTransport::new(&mailer_config)
            .await
            .expect("Failed to create SMTP transport");

        let dependency_container =
            DependencyContainer::new(postgres_db, smtp_transport);

        let app = app
            .di_module(dependency_container.module)
            .expect("Failed to load DI module")
            .controller::<UserController>()
            .controller::<AsignaturesController>();

        TestServer::new(app.router()).expect("Failed to start test server")
    }
}
