pub mod handlers;
pub mod utils;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use http::header::CONTENT_TYPE;
use tower_http::cors::CorsLayer;
use http::{Method, StatusCode};

use crate::state::AppState;
use crate::config::constants::{ALLOWED_ORIGINS};


fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(ALLOWED_ORIGINS.clone())
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE])
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::root_handler))
        .route("/execute/", post(handlers::single_execution)
            .options(|| async { StatusCode::OK }))
        .route("/v2/execute/", post(handlers::multi_execution)
            .options(|| async { StatusCode::OK }))
        .layer(create_cors_layer())
        .with_state(state)
}
