use async_trait::async_trait;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct ExamSession {
    pub id: Uuid,
    pub service_access_key_id: String,
    pub session_secret: String,
    pub challenge: String,
    pub difficulty: i16,
    pub expires_at: OffsetDateTime,
    pub action: Option<String>,
    pub created_at: OffsetDateTime,
}

#[async_trait]
pub trait ExamSessionRepository {
    async fn create(&self, session: &ExamSession) -> Result<ExamSession, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamSession>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
