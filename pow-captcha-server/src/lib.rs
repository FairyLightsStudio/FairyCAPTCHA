pub mod application;
pub mod domain;
pub mod error;
pub mod proto;
pub mod infrastructure;

use std::sync::Arc;
use sqlx::{Postgres, Pool};
use crate::application::captcha_service::CaptchaService;
use crate::infrastructure::grpc_services::GrpcService;
use crate::infrastructure::repositories::postgres::{PostgresExamSessionRepository, PostgresWorkProofTokenRepository};

#[derive(Clone)]
pub struct S {
    pub grpc_service: GrpcService<PostgresExamSessionRepository, PostgresWorkProofTokenRepository>,
}

impl S {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        let exam_session_repo = Arc::new(PostgresExamSessionRepository::new(db_pool.clone()));
        let work_proof_token_repo =
            Arc::new(PostgresWorkProofTokenRepository::new(db_pool.clone()));
        let captcha_service =
            Arc::new(CaptchaService::new(exam_session_repo, work_proof_token_repo));
        let grpc_service = GrpcService::new(captcha_service);
        Self { grpc_service }
    }
}
