use std::fs;
use std::env;
use std::string::ToString;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::models::LangConfig;

pub static HOST: Lazy<String> = Lazy::new(||
    env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
);
pub static PORT: Lazy<String> = Lazy::new(||
    env::var("PORT").unwrap_or_else(|_| "8000".to_string())
);

pub static WORKER_COUNT: Lazy<i8> = Lazy::new(||
    env::var("WORKER_COUNT").map(|v| v.parse().unwrap_or(8))
        .unwrap_or(4)
);

pub static LANG_CONFIG: Lazy<HashMap<String, LangConfig>> = Lazy::new(|| {
    let text = fs::read_to_string("config.json")
        .expect("config.json missing");

    serde_json::from_str(&text)
        .expect("invalid config.json")
});

pub static IS_DEBUG: Lazy<bool> = Lazy::new(|| {
    let val = env::var("RUST_LOG").unwrap_or_default().to_lowercase();
    val == "debug" || val == "full" || val == "trace"
});

