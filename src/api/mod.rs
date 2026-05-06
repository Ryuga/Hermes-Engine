pub mod handlers;


use std::env;
use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use http::header::CONTENT_TYPE;
use tower_http::cors::CorsLayer;
use http::{HeaderValue, Method, StatusCode};

use crate::state::AppState;


fn create_cors_layer() -> CorsLayer {
    let allowed_origin = env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let origin: HeaderValue = allowed_origin
        .parse().expect("Invalid ALLOWED_ORIGIN value");

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE])

}
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::root_handler))
        .route("/execute/", post(handlers::execution_handler)
            .options(|| async { StatusCode::OK }))
        .layer(create_cors_layer())
        .with_state(state)
}
