pub mod config;

pub mod features {

    pub mod users {
        pub mod domain;

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

    pub mod inscriptions {
        pub mod domain;

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
            pub use models::{
                InscriptionModel, InscriptionResponseModel, StudentEvaluationModel,
            };
            pub use repository::PostgresInscriptionRepository;
        }
    }
}

pub use features::{asignatures, inscriptions, users};

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
    pub mod validators {
        use validator::ValidationError;

        pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
            if uuid.is_empty() {
                return Err(ValidationError::new(
                    "La identificación no puede estar vacía.",
                ));
            }

            if uuid::Uuid::parse_str(uuid).is_err() {
                return Err(ValidationError::new("Identificación inválida."));
            }

            Ok(())
        }
    }
}
