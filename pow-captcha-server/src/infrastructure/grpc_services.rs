use std::sync::Arc;

use crate::application::captcha_service::CaptchaService;
use crate::domain::{ExamSessionRepository, WorkProofTokenRepository};
use crate::error::AppError;
use volo_gen::pow_captcha::v1::{
    GetChallengeRequest, GetChallengeResponse, PoWcaptchaBackendService, PoWcaptchaFrontendService,
    SubmitSolutionRequest, SubmitSolutionResponse, ValidateTokenRequest, ValidateTokenResponse,
};

pub struct GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    captcha_service: Arc<CaptchaService<ESR, WPR>>,
}

impl<ESR, WPR> GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    pub fn new(captcha_service: Arc<CaptchaService<ESR, WPR>>) -> Self {
        Self { captcha_service }
    }
}

impl<ESR, WPR> Clone for GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            captcha_service: self.captcha_service.clone(),
        }
    }
}

impl<ESR, WPR> PoWcaptchaFrontendService for GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    async fn get_challenge(
        &self,
        _req: ::volo_grpc::Request<GetChallengeRequest>,
    ) -> ::std::result::Result<::volo_grpc::Response<GetChallengeResponse>, ::volo_grpc::Status>
    {
        let result = self.captcha_service.get_challenge().await;
        result
            .map(|resp| ::volo_grpc::Response::new(resp))
            .map_err(AppError::into)
    }

    async fn submit_solution(
        &self,
        req: ::volo_grpc::Request<SubmitSolutionRequest>,
    ) -> ::std::result::Result<::volo_grpc::Response<SubmitSolutionResponse>, ::volo_grpc::Status>
    {
        let req = req.get_ref();
        let exam_session = req.exam_session.as_ref().ok_or_else(|| ::volo_grpc::Status::invalid_argument("Missing exam_session"))?;
        let solution = req.solution.as_ref().ok_or_else(|| ::volo_grpc::Status::invalid_argument("Missing solution"))?;

        let result = self.captcha_service.submit_solution(
            &exam_session.access_key_id,
            &exam_session.access_key_secret,
            &solution.nonce,
        ).await;

        result
            .map(|resp| ::volo_grpc::Response::new(resp))
            .map_err(AppError::into)
    }
}

impl<ESR, WPR> PoWcaptchaBackendService for GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    async fn validate_token(
        &self,
        req: ::volo_grpc::Request<ValidateTokenRequest>,
    ) -> ::std::result::Result<::volo_grpc::Response<ValidateTokenResponse>, ::volo_grpc::Status>
    {
        let token = &req.get_ref().token;
        let result = self.captcha_service.validate_token(token).await;
        result
            .map(|resp| ::volo_grpc::Response::new(resp))
            .map_err(AppError::into)
    }
}
