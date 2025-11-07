use std::sync::Arc;
use sword::core::injectable;
use uuid::Uuid;

mod report;

pub use report::PracticeReportService;

use crate::{
    courses::CourseService,
    enrollments::*,
    practices::*,
    shared::{
        AppResult, NotFoundError,
        event_queue::{Event, EventQueue},
    },
};

#[injectable]
pub struct PracticeService {
    practices: Arc<PracticeRepository>,
    enrollments: Arc<EnrollmentService>,
    courses: Arc<CourseService>,
    event_queue: Arc<EventQueue>,
}

impl PracticeService {
    pub async fn get_by_id(&self, id: &Uuid) -> AppResult<Option<Practice>> {
        self.practices.find_by_id(id).await
    }

    pub async fn create(
        &self,
        enrollment_id: &Uuid,
        input: CreatePracticeDto,
    ) -> AppResult<Practice> {
        let practice = Practice::from(input);

        let (enrollment, student, _, _) =
            self.enrollments.get_by_id(enrollment_id).await?;

        let (course, _) = self.courses.get_by_id(&enrollment.course_id).await?;

        let practice = self.practices.save(practice).await?;

        let enrollment = {
            let data = UpdateEnrollmentDto {
                practice_id: Some(practice.id.to_string()),
                student_scores: None,
            };

            self.enrollments.update(enrollment_id, data).await?
        };

        let event_data = (student, practice.clone(), course, enrollment);

        self.event_queue
            .publish(Event::PracticeCreated(event_data))
            .await;

        Ok(practice)
    }

    pub async fn update_status(
        &self,
        enrollment_id: &Uuid,
        practice_id: &Uuid,
        status: PracticeStatus,
    ) -> AppResult<Practice> {
        let (enrollment, student, practice, _) =
            self.enrollments.get_by_id(enrollment_id).await?;

        let mut practice = practice.ok_or(NotFoundError::practice(*practice_id))?;

        if practice.id != *practice_id {
            return Err(NotFoundError::practice(*practice_id))?;
        }

        let (course, teacher) =
            self.courses.get_by_id(&enrollment.course_id).await?;

        let event_data = (student, enrollment, practice.clone(), course, teacher);

        match status {
            PracticeStatus::Approved => {
                practice.practice_status = PracticeStatus::Approved;
                self.event_queue
                    .publish(Event::PracticeApproved(event_data))
                    .await;
            }

            PracticeStatus::Declined => {
                practice.practice_status = PracticeStatus::Declined;
                self.event_queue
                    .publish(Event::PracticeDeclined(event_data))
                    .await;
            }

            PracticeStatus::Pending => {}
        }

        self.practices.save(practice).await
    }

    pub async fn authorize(
        &self,
        enrollment_id: &Uuid,
        document: Vec<u8>,
    ) -> AppResult<()> {
        let (enrollment, student, practice, _) =
            self.enrollments.get_by_id(enrollment_id).await?;

        let practice = practice.ok_or(NotFoundError::practice(*enrollment_id))?;

        let (course, teacher) =
            self.courses.get_by_id(&enrollment.course_id).await?;

        let event_data = (student, course, teacher, practice, document);

        self.event_queue
            .publish(Event::PracticeAuthorized(event_data))
            .await;

        Ok(())
    }

    pub async fn update(
        &self,
        id: &Uuid,
        input: UpdatePracticeDto,
    ) -> AppResult<Practice> {
        let mut practice = self
            .practices
            .find_by_id(id)
            .await?
            .ok_or(NotFoundError::practice(*id))?;

        if let Some(enterprise_name) = input.enterprise_name {
            practice.enterprise_name = enterprise_name;
        }

        if let Some(description) = input.description {
            practice.description = description;
        }

        if let Some(location) = input.location {
            practice.location = location;
        }

        if let Some(supervisor_name) = input.supervisor_name {
            practice.supervisor_name = supervisor_name;
        }

        if let Some(supervisor_email) = input.supervisor_email {
            practice.supervisor_email = supervisor_email;
        }

        if let Some(start_date) = input.start_date {
            practice.start_date = Some(start_date);
        }

        if let Some(end_date) = input.end_date {
            practice.end_date = Some(end_date);
        }

        self.practices.save(practice).await
    }

    pub async fn evaluate(
        &self,
        enrollment_id: &Uuid,
        practice_id: &Uuid,
        evaluation_id: &Uuid,
        input: EvaluatePracticeDto,
    ) -> AppResult<Practice> {
        let (mut enrollment, student, practice, _) =
            self.enrollments.get_by_id(enrollment_id).await?;

        let (course, teacher) =
            self.courses.get_by_id(&enrollment.course_id).await?;

        let updated_evaluation = enrollment
            .student_scores
            .iter_mut()
            .find(|s| s.evaluation_id == *evaluation_id);

        let practice = practice.ok_or(NotFoundError::practice(*practice_id))?;

        if let Some(evaluation) = updated_evaluation {
            evaluation.score = input.score;
        }

        let student_scores_dto = enrollment
            .student_scores
            .iter()
            .map(|score| StudentScoreDto::from(score.clone()))
            .collect();

        let update_data = UpdateEnrollmentDto {
            practice_id: None,
            student_scores: Some(student_scores_dto),
        };

        self.enrollments.update(enrollment_id, update_data).await?;

        let event_data = (student, practice.clone(), course, teacher, input.score);

        self.event_queue
            .publish(Event::PracticeEvaluated(event_data))
            .await;

        Ok(practice)
    }

    pub async fn remove(&self, id: &Uuid) -> AppResult<()> {
        let practice = self
            .practices
            .find_by_id(id)
            .await?
            .ok_or(NotFoundError::practice(*id))?;

        self.practices.delete(&practice.id).await
    }
}
