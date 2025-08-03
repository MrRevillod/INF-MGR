use shaku::module;

use crate::{courses, enrollments, shared::database::PostgresDatabase, users};

use services::{
    hasher::BcryptPasswordHasher,
    mailer::{LettreTransport, MailerService},
};

pub struct DependencyContainer {
    pub module: AppModule,
}

impl DependencyContainer {
    pub fn new(
        postgres_conn: PostgresDatabase,
        lettre_transport: LettreTransport,
    ) -> Self {
        let module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_conn.into())
            .with_component_parameters::<LettreTransport>(lettre_transport.into())
            .build();

        DependencyContainer { module }
    }
}

module! {
    pub AppModule {
        components = [
            PostgresDatabase,
            LettreTransport,

            BcryptPasswordHasher,
            MailerService,

            courses::PostgresCourseRepository,
            courses::CourseServiceImpl,

            enrollments::PostgresEnrollmentRepository,
            enrollments::EnrollmentServiceImpl,

            users::PostgresUserRepository,
            users::UserServiceImpl
        ],
        providers = []
    }
}
