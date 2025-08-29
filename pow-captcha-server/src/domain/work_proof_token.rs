use async_trait::async_trait;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct WorkProofToken {
    pub id: Uuid,
    pub proof_token: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

#[async_trait]
pub trait WorkProofTokenRepository {
    async fn create(&self, token: &WorkProofToken) -> Result<WorkProofToken, AppError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<WorkProofToken>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
