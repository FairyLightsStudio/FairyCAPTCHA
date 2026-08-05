fn main() {
    connectrpc_build::Config::new()
        .files(&[
            "../contracts/pow_captcha/v1/pow_captcha_backend.proto",
            "../contracts/pow_captcha/v1/pow_captcha_frontend.proto",
        ])
        .includes(&["../contracts"])
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
