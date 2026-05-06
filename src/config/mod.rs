pub mod models;
pub mod constants;
pub mod utils;


use dotenvy::dotenv;
use tracing_subscriber::{fmt, EnvFilter};

pub fn bootstrap() {
    if let Err(e) = dotenv() {
        eprint!("No .env file found or error loading it: {}", e);
    }

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    tracing::info!("Hermes bootstrap complete: Environment and Tracing initialized.");
}