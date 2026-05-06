mod languages;
mod api;
mod config;
mod core;
mod state;

use std::env;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use state::AppState;
use core::workers::BoxManager;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::info;
use std::sync::Arc;

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


    let box_manager = Arc::new(BoxManager::new(8));

    let state = Arc::new(AppState::new(box_manager));

    let app = api::create_router(state);

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("listening on {}", addr);
    info!("Sandbox engine started");

    axum::serve(listener, app).await.unwrap();
}


