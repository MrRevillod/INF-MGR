use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    enrollments::EnrollmentService,
    imports::{ImportCourseDto, ImportUserDto, ImportedUser},
    shared::{
        services::{
            event_queue::{Event, EventQueue},
            hasher::PasswordHasher,
        },
        AppError,
    },
    user_filter,
    users::{Role, User, UserFilter, UserRepository},
};

#[derive(Component)]
#[shaku(interface = ImportService)]
pub struct ImportServiceImpl {
    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    enrollments: Arc<dyn EnrollmentService>,

    #[shaku(inject)]
    hasher: Arc<dyn PasswordHasher>,

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,
}

#[async_trait]
pub trait ImportService: Interface {
    async fn import_course_students(&self, data: ImportCourseDto) -> Result<(), AppError>;
    async fn classify_imported_students(
        &self,
        students: Vec<ImportUserDto>,
    ) -> Result<(Vec<ImportedUser>, Vec<User>), AppError>;
}

#[async_trait]
impl ImportService for ImportServiceImpl {
    async fn classify_imported_students(
        &self,
        students: Vec<ImportUserDto>,
    ) -> Result<(Vec<ImportedUser>, Vec<User>), AppError> {
        let ruts = students.iter().map(|s| s.rut.clone()).collect::<Vec<_>>();
        let existing_students = self.users.find_many(user_filter! { ruts }).await?;

        // Filter those not in existing students (by rut)
        // and map them to ImportedUser (preserving plain password for events)

        let imported_students = students
            .iter()
            .filter(|s| !existing_students.iter().any(|existing| existing.rut == s.rut))
            .map(|data| -> Result<ImportedUser, AppError> {
                let (plain, hash) = self.hasher.random_password()?;

                let entity = User {
                    rut: data.rut.clone(),
                    email: data.email.clone(),
                    name: data.name.clone(),
                    password: hash,
                    roles: vec![Role::Student],
                    ..Default::default()
                };

                Ok(ImportedUser {
                    entity,
                    plain_password: plain,
                })
            })
            .collect::<Result<Vec<ImportedUser>, AppError>>()?;

        Ok((imported_students, existing_students))
    }

    async fn import_course_students(&self, course: ImportCourseDto) -> Result<(), AppError> {
        let (imported_students, existing_students) =
            self.classify_imported_students(course.students).await?;

        let event_data = imported_students
            .iter()
            .map(|s| (s.entity.name.clone(), s.entity.email.clone(), s.plain_password.clone()))
            .collect::<Vec<_>>();

        let new_students = imported_students.into_iter().map(|s| s.entity).collect::<Vec<User>>();

        self.users.create_many(new_students.clone()).await?;
        self.event_queue.publish(Event::ManyUsersCreated(event_data)).await;

        let all_students = existing_students
            .into_iter()
            .map(|s| s.id)
            .chain(new_students.into_iter().map(|s| s.id))
            .collect::<Vec<_>>();

        let course_id = Uuid::parse_str(&course.id).unwrap();

        self.enrollments.create_many(&course_id, all_students).await?;

        Ok(())
    }
}
