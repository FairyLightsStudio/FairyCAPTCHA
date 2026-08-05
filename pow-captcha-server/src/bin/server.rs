use std::net::SocketAddr;
use std::sync::Arc;

use connectrpc::Router;
use sqlx::postgres::PgPoolOptions;

use pow_captcha::S;
use serde::Deserialize;

use pow_captcha::proto::pow_captcha::v1::{
    PoWcaptchaBackendService, PoWcaptchaFrontendService,
};

#[derive(Debug)]
struct ServerError(String);

impl core::fmt::Display for ServerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::error::Error for ServerError {}

#[derive(Deserialize)]
struct AppConfig {
    database: DBConfig,
    address: String,
}

#[derive(Deserialize)]
struct DBConfig {
    url: String,
}

#[tokio::main]
async fn main() -> exn::Result<(), ServerError> {
    use exn::ResultExt;

    let settings = config::Config::builder()
        .set_default("address", "[::]:8080")
        .or_raise(|| ServerError("Failed to set default value".into()))?
        .add_source(config::Environment::with_prefix("POWCAPTCHA_SERVICE").separator("_"))
        .build()
        .or_raise(|| ServerError("Failed to build config".into()))?;

    let app_config: AppConfig = settings
        .try_deserialize()
        .or_raise(|| ServerError("Failed to load configuration from environment variable".into()))?;

    let addr: SocketAddr = app_config
        .address
        .parse()
        .or_raise(|| {
            ServerError(format!("Address {} is not valid", app_config.address))
        })?;

    let db_pool = PgPoolOptions::new()
        .connect(&app_config.database.url)
        .await
        .or_raise(|| {
            ServerError(format!(
                "Database {} connect failed",
                app_config.database.url
            ))
        })?;

    let service = Arc::new(S::new(db_pool));

    let backend_svc = service.clone();
    let frontend_svc = service;

    let router = Router::new()
        .register(PoWcaptchaBackendService::register(backend_svc))
        .register(PoWcaptchaFrontendService::register(frontend_svc));

    let app = router.into_axum_router();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .or_raise(|| ServerError(format!("Failed to bind to {}", addr)))?;

    axum::serve(listener, app)
        .await
        .or_raise(|| ServerError("Server error".into()))?;

    Ok(())
}
