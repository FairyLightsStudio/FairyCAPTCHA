use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use volo_grpc::server::{Server, ServiceBuilder};

use pow_captcha::S;
use serde::Deserialize;
use anyhow::{Context, Result};

#[derive(Deserialize)]
struct AppConfig {
    database: DBConfig,
    address: String,
}
#[derive(Deserialize)]
struct DBConfig {
    url: String
}

#[volo::main]
async fn main() -> Result<()> {


    let settings = config::Config::builder()
        .set_default("address", "[::]:8080")?
        .add_source(config::Environment::with_prefix("POWCAPTCHA_SERVICE").separator("_"))
        .build()
       .unwrap();


    let app_config: AppConfig = settings.try_deserialize().context("Failed to load configuration from environment variable")?;
    
    let addr: SocketAddr = app_config.address.parse().context(format!("address {} not valid", app_config.address))?;
    let addr = volo::net::Address::from(addr);

    let db_pool = PgPoolOptions::new()
    .connect(&app_config.database.url).await.context(format!("database {} connect failed", app_config.database.url))?;

    let service = S::new(db_pool);

    Server::new()
        .add_service(
            ServiceBuilder::new(volo_gen::pow_captcha::v1::PoWcaptchaBackendServiceServer::new(service.clone()))
                .build(),
        )
        .add_service(
            ServiceBuilder::new(volo_gen::pow_captcha::v1::PoWcaptchaFrontendServiceServer::new(service))
                .build(),
        )
        .run(addr)
        .await
        .unwrap();
    
    Ok(())
}
