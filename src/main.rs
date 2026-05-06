mod languages;
mod api;
mod config;
mod core;
mod state;

use tokio::net::TcpListener;
use state::AppState;
use tracing::info;
use std::sync::Arc;
use config::constants;

#[tokio::main(flavor="multi_thread")]
async fn main() {
    config::bootstrap();

    let state = Arc::new(AppState::new(*constants::WORKER_COUNT));
    let app = api::create_router(state);

    let addr = format!("{}:{}", *constants::HOST, *constants::PORT);
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Hermes Engine listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}


