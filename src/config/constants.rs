
use std::env;
use std::fs;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::config::models::LangConfig;

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

