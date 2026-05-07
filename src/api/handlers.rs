use std::sync::Arc;

use axum::Json;
use tracing::{debug, error};
use http::StatusCode;
use axum::extract::State;

use crate::api::utils::ValidatedJson;
use crate::state::{AppState};
use crate::core::exe::execute_code;
use crate::config::models::{File, Req, ReqMulti, Resp};
use crate::config::utils::get_lang_config;

pub async fn root_handler() -> &'static str {
    "UP!"
}

pub async fn single_execution(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    Json(req): Json<Req>
) -> Result<Json<Resp>, StatusCode> {
    debug!("Single Execution request: {:?}", req);

    let state_ref = Arc::clone(&state);
    let lang_config = get_lang_config(&req.language).map_err(|_| StatusCode::BAD_REQUEST)?;

    let auth_token = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let file = File {
        name: lang_config.source.clone(),
        content: req.code
    };

    let multi_req = ReqMulti {
        entry_file:lang_config.source.clone(),
        files: [file].to_vec(),
        language: req.language,
    };

    let result = run_execution(state_ref, multi_req, auth_token).await?;
    Ok(Json(result))
}

pub async fn multi_execution(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    ValidatedJson(req): ValidatedJson<ReqMulti>
) -> Result<Json<Resp>, StatusCode> {
    debug!("Multi Execution request: {:?}", req);

    let state_ref = Arc::clone(&state);

    let auth_token = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let result = run_execution(state_ref, req, auth_token).await?;
    Ok(Json(result))
}


async fn run_execution(state: Arc<AppState>,
    req: ReqMulti,
    auth_token: Option<String>
) -> Result<Resp, StatusCode> {

    tokio::task::spawn_blocking(move || {
        let manager = &state.box_manager;

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
        })
}
