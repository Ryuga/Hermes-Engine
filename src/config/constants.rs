use std::fs;
use std::env;
use std::string::ToString;
use std::collections::HashMap;
use http::HeaderValue;
use once_cell::sync::Lazy;
use crate::utils::misc::get_calculated_worker_count;
use super::models::LangConfig;

pub static HOST: Lazy<String> = Lazy::new(||
    env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
);
pub static PORT: Lazy<String> = Lazy::new(||
    env::var("PORT").unwrap_or_else(|_| "8000".to_string())
);

pub static WORKER_COUNT: Lazy<usize> = Lazy::new(||
    env::var("WORKER_COUNT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(get_calculated_worker_count)
);

pub static ALLOWED_ORIGINS: Lazy<Vec<HeaderValue>> = Lazy::new(|| {
    env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .split(',')
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<HeaderValue>().ok())
        .collect()
});

pub static LANG_CONFIG: Lazy<HashMap<String, LangConfig>> = Lazy::new(|| {
    let text = fs::read_to_string("config.json")
        .expect("config.json missing");

    serde_json::from_str(&text)
        .expect("invalid config.json")
});

pub static IS_DEBUG: Lazy<bool> = Lazy::new(|| {
    let val = env::var("RUST_LOG").unwrap_or_default().to_lowercase();
    val.contains("debug") || val.contains("full") || val.contains("trace")
});

