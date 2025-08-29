use async_trait::async_trait;
use sqlx::{Postgres, Pool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{ExamSession, ExamSessionRepository, WorkProofToken, WorkProofTokenRepository};
use crate::error::AppError;

#[derive(sqlx::FromRow, Debug)]
struct ExamSessionRow {
    id: Uuid,
    service_access_key_id: String,
    session_secret: String,
    challenge: String,
    difficulty: i16,
    expires_at: OffsetDateTime,
    action: Option<String>,
    created_at: OffsetDateTime,
}

impl From<ExamSessionRow> for ExamSession {
    fn from(row: ExamSessionRow) -> Self {
        Self {
            id: row.id,
            service_access_key_id: row.service_access_key_id,
            session_secret: row.session_secret,
            challenge: row.challenge,
            difficulty: row.difficulty,
            expires_at: row.expires_at,
            action: row.action,
            created_at: row.created_at,
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
struct WorkProofTokenRow {
    id: Uuid,
    proof_token: String,
    expires_at: OffsetDateTime,
    created_at: OffsetDateTime,
}

impl From<WorkProofTokenRow> for WorkProofToken {
    fn from(row: WorkProofTokenRow) -> Self {
        Self {
            id: row.id,
            proof_token: row.proof_token,
            expires_at: row.expires_at,
            created_at: row.created_at,
        }
    }
}

pub struct PostgresExamSessionRepository {
    pool: Pool<Postgres>,
}

impl PostgresExamSessionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExamSessionRepository for PostgresExamSessionRepository {
    async fn create(&self, session: &ExamSession) -> Result<ExamSession, AppError> {
        let row = sqlx::query_as::<_, ExamSessionRow>(
            "INSERT INTO exam_sessions (id, service_access_key_id, session_secret, challenge, difficulty, expires_at, action, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
        )
        .bind(session.id)
        .bind(&session.service_access_key_id)
        .bind(&session.session_secret)
        .bind(&session.challenge)
        .bind(session.difficulty)
        .bind(session.expires_at)
        .bind(&session.action)
        .bind(session.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamSession>, AppError> {
        let row = sqlx::query_as::<_, ExamSessionRow>("SELECT * FROM exam_sessions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM exam_sessions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

pub struct PostgresWorkProofTokenRepository {
    pool: Pool<Postgres>,
}

impl PostgresWorkProofTokenRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkProofTokenRepository for PostgresWorkProofTokenRepository {
    async fn create(&self, token: &WorkProofToken) -> Result<WorkProofToken, AppError> {
        let row = sqlx::query_as::<_, WorkProofTokenRow>(
            "INSERT INTO work_proof_tokens (id, proof_token, expires_at, created_at) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(token.id)
        .bind(&token.proof_token)
        .bind(token.expires_at)
        .bind(token.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<WorkProofToken>, AppError> {
        let row = sqlx::query_as::<_, WorkProofTokenRow>("SELECT * FROM work_proof_tokens WHERE proof_token = $1")
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM work_proof_tokens WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}