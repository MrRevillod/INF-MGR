use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::courses::CourseRepository;

use crate::inscriptions::{
    CreateInscriptionDto, Inscription, InscriptionError, InscriptionFilter,
    InscriptionRepository, InscriptionWithCourse, StudentScore,
    UpdateInscriptionDto,
};

use crate::users::UserRepository;

#[derive(Component)]
#[shaku(interface = InscriptionService)]
pub struct InscriptionServiceImpl {
    #[shaku(inject)]
    inscriptions: Arc<dyn InscriptionRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    courses: Arc<dyn CourseRepository>,
}

#[async_trait]
pub trait InscriptionService: Interface {
    async fn get_all(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<InscriptionWithCourse>, InscriptionError>;

    async fn create(
        &self,
        input: CreateInscriptionDto,
    ) -> Result<Inscription, InscriptionError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateInscriptionDto,
    ) -> Result<Inscription, InscriptionError>;

    async fn remove(&self, id: &Uuid) -> Result<(), InscriptionError>;
}

#[async_trait]
impl InscriptionService for InscriptionServiceImpl {
    async fn get_all(
        &self,
        filter: InscriptionFilter,
    ) -> Result<Vec<InscriptionWithCourse>, InscriptionError> {
        let mut results = Vec::new();
        let inscriptions = self.inscriptions.find_all(filter).await?;

        for inscription in inscriptions {
            let Some(course) = self
                .courses
                .find_by_id(&inscription.course_id)
                .await
                .map_err(|e| InscriptionError::ForeignCourseError(e.to_string()))?
            else {
                return Err(InscriptionError::AsignatureNotFound);
            };

            results.push((inscription, course));
        }

        Ok(results)
    }

    async fn create(
        &self,
        input: CreateInscriptionDto,
    ) -> Result<Inscription, InscriptionError> {
        let inscription = Inscription::from(input);

        let filter = InscriptionFilter {
            student_id: Some(inscription.student_id),
            course_id: Some(inscription.course_id),
        };

        if !self.inscriptions.find_all(filter).await?.is_empty() {
            return Err(InscriptionError::InscriptionAlreadyExists);
        }

        let (student_exists, course_exists) = tokio::join!(
            self.users.find_by_id(&inscription.student_id),
            self.courses.find_by_id(&inscription.course_id)
        );

        let (student_exists, course_exists) = (
            student_exists
                .map_err(|e| InscriptionError::ForeignUserError(e.to_string()))?,
            course_exists
                .map_err(|e| InscriptionError::ForeignCourseError(e.to_string()))?,
        );

        let Some(student) = student_exists else {
            return Err(InscriptionError::StudentNotFound);
        };

        if course_exists.is_none() {
            return Err(InscriptionError::AsignatureNotFound);
        };

        if !student.is_student() {
            return Err(InscriptionError::InvalidStudentRole);
        }

        self.inscriptions.save(inscription).await
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateInscriptionDto,
    ) -> Result<Inscription, InscriptionError> {
        let Some(mut inscription) = self.inscriptions.find_by_id(id).await? else {
            return Err(InscriptionError::NotFound);
        };

        if let Some(scores) = input.student_scores {
            inscription.student_scores =
                scores.into_iter().map(StudentScore::from).collect();
        }

        self.inscriptions.save(inscription).await
    }

    async fn remove(&self, id: &Uuid) -> Result<(), InscriptionError> {
        if self.inscriptions.find_by_id(id).await?.is_none() {
            return Err(InscriptionError::NotFound);
        };

        self.inscriptions.delete(id).await
    }
}
