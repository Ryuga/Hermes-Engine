use std::sync::Arc;

use axum::Json;
use tracing::{debug, error};
use http::StatusCode;
use axum::extract::State;
use crate::api::utils::ValidatedJson;
use crate::state::{AppState};
use crate::core::exe::execute_code;
use crate::config::models::{ReqMulti, Resp};


pub async fn root_handler() -> &'static str {
    "UP!"
}

pub async fn execution_handler(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    ValidatedJson(req): ValidatedJson<ReqMulti>
) -> Result<Json<Resp>, StatusCode> {
    debug!("Received request: {:?}", req);

    let state_ref = Arc::clone(&state);

    let auth_token = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let result = tokio::task::spawn_blocking(move || {
        let manager = &state_ref.box_manager;

        let isolate_box = manager.acquire();
        let run_result = execute_code(&isolate_box, req, auth_token);
        manager.release(isolate_box);
        run_result
    }).await
        .map_err(|e| {
            error!("Task error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|e| {
            error!("Exec error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}
