use std::sync::Arc;
use std::time::Duration;

use pow_captcha_core::{generate_challenge, verify_solution, HashAlgorithm as CoreAlgorithm, Challenge as CoreChallenge, Solution as CoreSolution};
use rand::RngCore;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{ExamSession, ExamSessionRepository, WorkProofToken, WorkProofTokenRepository};
use crate::error::AppError;
use crate::proto::pow_captcha::v1::{GetChallengeResponse, SubmitSolutionResponse, ValidateTokenResponse, Session, Challenge};

pub struct CaptchaService<ESR, WPR>
where
    ESR: ExamSessionRepository,
    WPR: WorkProofTokenRepository,
{
    exam_session_repo: Arc<ESR>,
    work_proof_token_repo: Arc<WPR>,
}

impl<ESR, WPR> CaptchaService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync,
    WPR: WorkProofTokenRepository + Send + Sync,
{
    pub fn new(exam_session_repo: Arc<ESR>, work_proof_token_repo: Arc<WPR>) -> Self {
        Self {
            exam_session_repo,
            work_proof_token_repo,
        }
    }

    pub async fn get_challenge(&self) -> Result<GetChallengeResponse, AppError> {
        let core_challenge = generate_challenge(Some(20), Some(CoreAlgorithm::Sha256));
        let mut session_secret_bytes = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut session_secret_bytes);
        let session_secret = hex::encode(session_secret_bytes);

        let expires_at = OffsetDateTime::now_utc() + Duration::from_secs(300);

        let session = ExamSession {
            id: Uuid::new_v4(),
            service_access_key_id: "service_key_id_placeholder".to_string(), // TODO
            session_secret,
            challenge: core_challenge.puzzle.clone(),
            difficulty: core_challenge.difficulty as i16,
            expires_at,
            action: None,
            created_at: OffsetDateTime::now_utc(),
        };

        let created_session = self.exam_session_repo.create(&session).await?;

        let response_session = Session {
            access_key_id: created_session.id.to_string().into(),
            access_key_secret: created_session.session_secret.into(),
            ..Default::default()
        };

        let response_challenge = Challenge {
            algorithm: "sha256".into(),
            base_data: core_challenge.puzzle.into(),
            difficulty: core_challenge.difficulty as i32,
            timestamp: Some(prost_types::Timestamp {
                seconds: created_session.created_at.unix_timestamp(),
                nanos: 0,
            }),
            expires_on: Some(prost_types::Timestamp {
                seconds: created_session.expires_at.unix_timestamp(),
                nanos: 0,
            }),
            ..Default::default()
        };

        Ok(GetChallengeResponse {
            exam_session: Some(response_session),
            challenge: Some(response_challenge),
        })
    }

    pub async fn submit_solution(
        &self,
        session_id_str: &str,
        session_secret: &str,
        solution_nonce: &str,
    ) -> Result<SubmitSolutionResponse, AppError> {
        let session_id = Uuid::parse_str(session_id_str)
            .map_err(|_| AppError::InvalidArgument("Invalid session_id format".to_string()))?;

        let session = self
            .exam_session_repo
            .find_by_id(session_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;

        if session.expires_at < OffsetDateTime::now_utc() {
            return Err(AppError::DeadlineExceeded("Challenge expired".to_string()));
        }

        if session.session_secret != session_secret {
            return Err(AppError::Unauthorized("Invalid session secret".to_string()));
        }

        let core_challenge = CoreChallenge {
            hash_algorithm: CoreAlgorithm::Sha256,
            puzzle: session.challenge.clone(),
            difficulty: session.difficulty as usize,
        };
        let core_solution = CoreSolution {
            nonce: solution_nonce.parse().map_err(|_| AppError::InvalidArgument("Invalid nonce format".to_string()))?,
        };

        if !verify_solution(&core_challenge, &core_solution) {
            return Err(AppError::InvalidArgument("Invalid solution".to_string()));
        }

        let mut token_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut token_bytes);
        let token_str = hex::encode(token_bytes);
        let token_expires_at = OffsetDateTime::now_utc() + Duration::from_secs(600);

        let work_proof_token = WorkProofToken {
            id: Uuid::new_v4(),
            proof_token: token_str.clone(),
            expires_at: token_expires_at,
            created_at: OffsetDateTime::now_utc(),
        };

        self.work_proof_token_repo.create(&work_proof_token).await?;
        self.exam_session_repo.delete(session_id).await?;

        Ok(SubmitSolutionResponse {
            token: token_str.into(),
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<ValidateTokenResponse, AppError> {
        let token_record = match self.work_proof_token_repo.find_by_token(token).await? {
            Some(record) => record,
            None => {
                return Ok(ValidateTokenResponse {
                    valid: false,
                    ..Default::default()
                })
            }
        };

        if token_record.expires_at < OffsetDateTime::now_utc() {
            return Ok(ValidateTokenResponse {
                valid: false,
                ..Default::default()
            });
        }

        self.work_proof_token_repo.delete(token_record.id).await?;

        Ok(ValidateTokenResponse {
            valid: true,
            ..Default::default()
        })
    }
}
