pub mod application;
pub mod domain;
pub mod error;
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
        let work_proof_token_repo = Arc::new(PostgresWorkProofTokenRepository::new(db_pool.clone()));
        let captcha_service = Arc::new(CaptchaService::new(exam_session_repo, work_proof_token_repo));
        let grpc_service = GrpcService::new(captcha_service);
        Self { grpc_service }
    }
}

impl volo_gen::pow_captcha::v1::PoWcaptchaBackendService for S {
    async fn validate_token(
        &self,
        req: ::volo_grpc::Request<volo_gen::pow_captcha::v1::ValidateTokenRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::pow_captcha::v1::ValidateTokenResponse>,
        ::volo_grpc::Status,
    > {
        self.grpc_service.validate_token(req).await
    }
}

impl volo_gen::pow_captcha::v1::PoWcaptchaFrontendService for S {
    async fn get_challenge(
        &self,
        req: ::volo_grpc::Request<volo_gen::pow_captcha::v1::GetChallengeRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::pow_captcha::v1::GetChallengeResponse>,
        ::volo_grpc::Status,
    > {
        self.grpc_service.get_challenge(req).await
    }

    async fn submit_solution(
        &self,
        req: ::volo_grpc::Request<volo_gen::pow_captcha::v1::SubmitSolutionRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::pow_captcha::v1::SubmitSolutionResponse>,
        ::volo_grpc::Status,
    > {
        self.grpc_service.submit_solution(req).await
    }
}
