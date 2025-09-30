use async_trait::async_trait;
use reqwest::Client as HttpClient;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::StudentsApiConfig,
    courses::{Course, CourseRepository},
    enrollments::{Enrollment, EnrollmentRepository},
    imports::ImportedStudent,
    shared::{
        AppError, NotFoundError,
        services::{Event, EventQueue},
    },
    users::*,
};

#[derive(Component)]
#[shaku(interface = ImportService)]
pub struct ImportServiceImpl {
    #[shaku(inject)]
    courses: Arc<dyn CourseRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,

    #[shaku(inject)]
    enrollments: Arc<dyn EnrollmentRepository>,
}

#[async_trait]
pub trait ImportService: Interface {
    async fn import_course_students(
        &self,
        config: StudentsApiConfig,
        course_id: &Uuid,
    ) -> Result<(), AppError>;

    async fn get_students(
        &self,
        config: StudentsApiConfig,
        course: &Course,
    ) -> Result<Vec<ImportedStudent>, AppError>;

    async fn classify_imported_students(
        &self,
        students: Vec<ImportedStudent>,
    ) -> Result<(Vec<ImportedStudent>, Vec<User>), AppError>;
}

#[async_trait]
impl ImportService for ImportServiceImpl {
    async fn import_course_students(
        &self,
        config: StudentsApiConfig,
        course_id: &Uuid,
    ) -> Result<(), AppError> {
        let course = self
            .courses
            .find_by_id(course_id)
            .await?
            .ok_or(NotFoundError::course(*course_id))?;

        let students = self.get_students(config, &course).await?;

        let (imported_students, existing_students) =
            self.classify_imported_students(students).await?;

        let mut new_users = vec![];

        for student in imported_students.iter() {
            let user = User {
                id: Uuid::new_v4(),
                name: student.name.clone(),
                email: student.email.clone(),
                rut: student.rut.clone(),
                role: Role::Student,
                ..Default::default()
            };

            new_users.push(user.clone());
            self.users.save(user).await?;
        }

        self.event_queue
            .publish(Event::ImportedStudents(imported_students))
            .await;

        let all_students = existing_students
            .into_iter()
            .chain(new_users)
            .collect::<Vec<_>>();

        let enrollments_to_create = all_students
            .iter()
            .map(|student| Enrollment {
                id: Uuid::new_v4(),
                course_id: *course_id,
                student_scores: vec![],
                student_id: student.id,
                practice_id: None,
            })
            .collect::<Vec<_>>();

        self.enrollments.create_many(enrollments_to_create).await?;

        Ok(())
    }

    async fn classify_imported_students(
        &self,
        students: Vec<ImportedStudent>,
    ) -> Result<(Vec<ImportedStudent>, Vec<User>), AppError> {
        let imported_ruts =
            students.iter().map(|s| s.rut.clone()).collect::<Vec<_>>();

        let existing_students = self
            .users
            .find_many(user_filter! { ruts: imported_ruts })
            .await?;

        // Filter those not in existing students (by rut)
        let existing_ruts = existing_students
            .iter()
            .map(|s| s.rut.clone())
            .collect::<Vec<_>>();

        let imported_students = students
            .into_iter()
            .filter(|s| !existing_ruts.contains(&s.rut))
            .collect::<Vec<_>>();

        Ok((imported_students, existing_students))
    }

    async fn get_students(
        &self,
        config: StudentsApiConfig,
        course: &Course,
    ) -> Result<Vec<ImportedStudent>, AppError> {
        let Course { year, code, .. } = course;
        let StudentsApiConfig { api_key, api_url } = config;

        let response = HttpClient::new()
            .get(format!("{api_url}/courses/{year}/{code}/students"))
            .header("x-api-key", api_key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Students API request error: {:?}", e);
                AppError::InternalServerError(e.into())
            })?;

        if !response.status().is_success() {
            tracing::error!(
                "Students API returned error status: {}",
                response.status()
            );
            return Err(AppError::InternalServerError(
                "Failed to fetch students from Students API".into(),
            ));
        }

        let students: Vec<ImportedStudent> = response.json().await.map_err(|e| {
            tracing::error!("Failed to parse Students API response: {:?}", e);
            AppError::InternalServerError(e.into())
        })?;

        Ok(students)
    }
}
