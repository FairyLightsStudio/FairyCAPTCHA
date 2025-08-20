pub struct S;

impl volo_gen::pow_captcha::v1::PoWcaptchaBackendService for S {
    async fn validate_token(
        &self,
        _req: ::volo_grpc::Request<volo_gen::pow_captcha::v1::ValidateTokenRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::pow_captcha::v1::ValidateTokenResponse>,
        ::volo_grpc::Status,
    > {
        ::std::result::Result::Ok(::volo_grpc::Response::new(Default::default()))
    }
}

impl volo_gen::pow_captcha::v1::PoWcaptchaFrontendService for S {
    async fn get_challenge(
        &self,
        _req: ::volo_grpc::Request<volo_gen::pow_captcha::v1::GetChallengeRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::pow_captcha::v1::GetChallengeResponse>,
        ::volo_grpc::Status,
    > {
        ::std::result::Result::Ok(::volo_grpc::Response::new(Default::default()))
    }

    async fn submit_solution(
        &self,
        _req: ::volo_grpc::Request<volo_gen::pow_captcha::v1::SubmitSolutionRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::pow_captcha::v1::SubmitSolutionResponse>,
        ::volo_grpc::Status,
    > {
        ::std::result::Result::Ok(::volo_grpc::Response::new(Default::default()))
    }
}
