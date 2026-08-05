use std::sync::Arc;

use buffa::view::OwnedView;
use connectrpc::{RequestContext, Response, ServiceResult};

use crate::application::captcha_service::CaptchaService;
use crate::domain::{ExamSessionRepository, WorkProofTokenRepository};
use crate::error::AppError;
use crate::proto::pow_captcha::v1::*;

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
        _ctx: RequestContext,
        _req: OwnedView<GetChallengeRequestView<'static>>,
    ) -> ServiceResult<GetChallengeResponse> {
        self.captcha_service
            .get_challenge()
            .await
            .map(Response::ok)
            .map_err(AppError::into)
    }

    async fn submit_solution(
        &self,
        _ctx: RequestContext,
        req: OwnedView<SubmitSolutionRequestView<'static>>,
    ) -> ServiceResult<SubmitSolutionResponse> {
        let exam_session = req
            .exam_session
            .as_ref()
            .ok_or_else(|| AppError::InvalidArgument("Missing exam_session".into()))?;
        let solution = req
            .solution
            .as_ref()
            .ok_or_else(|| AppError::InvalidArgument("Missing solution".into()))?;

        self.captcha_service
            .submit_solution(
                &exam_session.access_key_id,
                &exam_session.access_key_secret,
                &solution.nonce,
            )
            .await
            .map(Response::ok)
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
        _ctx: RequestContext,
        req: OwnedView<ValidateTokenRequestView<'static>>,
    ) -> ServiceResult<ValidateTokenResponse> {
        self.captcha_service
            .validate_token(&req.token)
            .await
            .map(Response::ok)
            .map_err(AppError::into)
    }
}
