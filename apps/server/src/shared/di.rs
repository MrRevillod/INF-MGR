use shaku::module;

use crate::{
    asignatures, inscriptions, practices, reports,
    shared::{
        database::PostgresDatabase,
        services::{BcryptPasswordHasher, MailerService},
        smtp::LettreTransport,
    },
    users,
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

            asignatures::infrastructure::PostgresAsignatureRepository,
            asignatures::application::GetAsignaturesCaseImpl,
            asignatures::application::CreateAsignatureCaseImpl,
            asignatures::application::UpdateAsignatureCaseImpl,
            asignatures::application::DeleteAsignatureCaseImpl,

            inscriptions::infrastructure::PostgresInscriptionRepository,
            inscriptions::application::GetInscriptionsCaseImpl,
            inscriptions::application::CreateInscriptionCaseImpl,
            inscriptions::application::UpdateInscriptionCaseImpl,
            inscriptions::application::DeleteInscriptionCaseImpl,

            practices::infrastructure::PostgresPracticeRepository,
            practices::application::GetPracticeCaseImpl,
            practices::application::CreatePracticeCaseImpl,
            practices::application::UpdatePracticeCaseImpl,
            practices::application::DeletePracticeCaseImpl,

            reports::infrastructure::PostgresReportRepository,
            reports::application::GetReportsCaseImpl,
            reports::application::UpdateReportCaseImpl,

            users::infrastructure::PostgresUserRepository,
            users::application::GetUsersCaseImpl,
            users::application::CreateUserCaseImpl,
            users::application::UpdateUserCaseImpl,
            users::application::DeleteUserCaseImpl
        ],
        providers = []
    }
}
