use serde::{Deserializer, Deserialize, de::Error};
use crate::config::models::LangConfig;
use super::constants::LANG_CONFIG;


pub fn string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        _ => Err(D::Error::custom("expected string or integer")),
    }
}

pub fn get_lang_config(lang: &str) -> &'static LangConfig {
    LANG_CONFIG.get(lang).unwrap_or_else(|| panic!("Unsupported language: {}", lang))
}
