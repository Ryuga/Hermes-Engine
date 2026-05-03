mod exe;
mod runner;
mod loader;
mod models;
mod languages;
mod workers;

use std::env;
use dotenvy::dotenv;
use axum::http::StatusCode;
use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}, Json};

use crate::models::{Resp, Req};
use crate::exe::execute_code;
use tower_http::cors::{CorsLayer};
use http::{Method, header::CONTENT_TYPE, HeaderValue};
use once_cell::sync::Lazy;
use tokio::sync::Semaphore;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::info;

static EXEC_LIMIT: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(4));

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

#[tokio::main(flavor="multi_thread", worker_threads=2)]
async fn main() {
    init_tracing();
    dotenv().ok();


    let port = env::var("PORT").unwrap_or("8000".into());
    let host = env::var("HOST").unwrap_or("127.0.0.1".into());
    let allowed_origin = env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let origin: HeaderValue = allowed_origin
        .parse().expect("Invalid ALLOWED_ORIGIN value");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(handler))
        .route(
            "/execute/", post(execution_handler).options(|| async { StatusCode::OK })
        )
        .layer(cors);

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("listening on {}", addr);
    info!("Sandbox engine started");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "UP!"
}

async fn execution_handler(
    headers: axum::http::HeaderMap,
    Json(req): Json<Req>
) -> Result<Json<Resp>, StatusCode> {
    let permit = EXEC_LIMIT.acquire().await.unwrap();

    let auth_token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let result = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        execute_code(req, auth_token)
    }).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(result))
}
