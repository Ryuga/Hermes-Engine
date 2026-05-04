use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use std::sync::Arc;
use crate::workers::BoxManager;

fn default_vector() -> Vec<String> { vec![] }
fn default_compile() -> bool { false }
fn default_authenticate() -> bool { false }
fn default_time_limit() -> u64 { 2 }
fn default_cpu_time_sec() -> u64 { 2 }
fn default_memory_kb() -> u64 { 256 * 1024 }
fn default_stack_kb() -> u64 { 64 * 1024 }
fn default_processes() -> u64 { 16 }
fn default_open_files() -> u64 { 64 }
fn default_file_size_kb() -> u64 { 1024 }


#[derive(Clone)]
pub struct LangConfig {
    pub source: String,

    pub compile: bool,

    pub authenticate: bool,

    pub compiler_path: String,
    pub compiler_args: Vec<String>,

    pub runtime_path: String,
    pub runtime_args: Vec<String>,

    pub max_time_limit: u64,
    pub max_cpu_time_sec: u64,
    pub max_memory_kb: u64,
    pub max_stack_kb: u64,
    pub max_processes: u64,
    pub max_open_files: u64,
    pub max_file_size_kb: u64,
    pub max_output_kb: u64,
}

#[derive(Deserialize)]
pub struct RawLangConfig {
    pub source: String,

    #[serde(default = "default_compile")]
    pub compile: bool,

    #[serde(default = "default_authenticate")]
    pub authenticate: bool,

    #[serde(default)]
    pub compiler_path: Option<String>,

    #[serde(default = "default_vector")]
    pub compiler_args: Vec<String>,

    pub runtime_path: String,

    #[serde(default = "default_vector")]
    pub runtime_args: Vec<String>,

    #[serde(default = "default_time_limit")]
    pub max_time_limit: String,

    #[serde(default = "default_cpu_time_sec")]
    pub max_cpu_time_sec: String,

    #[serde(default = "default_memory_kb")]
    pub max_memory_kb: String,

    #[serde(default = "default_stack_kb")]
    pub max_stack_kb: String,

    #[serde(default = "default_processes")]
    pub max_processes: String,

    #[serde(default = "default_open_files")]
    pub max_open_files: String,

    #[serde(default = "default_file_size_kb")]
    pub max_file_size_kb: String,

}


impl<'de> Deserialize<'de> for LangConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
        {
            let raw = RawLangConfig::deserialize(deserializer)?;

            if raw.source.trim().is_empty(){
                return Err(D::Error::custom("source can't be empty"));
            }

            if raw.compile && raw.compiler_path.is_none() {
                return Err(D::Error::custom(
                    "compiler_path is required when compile is set true",
                ));
            }

            Ok(LangConfig {
                source: raw.source,
                compile: raw.compile,
                authenticate: raw.authenticate,
                runtime_path: raw.runtime_path,
                compiler_path: raw.compiler_path.unwrap_or_default(),
                compiler_args: raw.compiler_args,
                runtime_args: raw.runtime_args,
                max_time_limit: raw.max_time_limit,
                max_cpu_time_sec: raw.max_cpu_time_sec,
                max_memory_kb: raw.max_memory_kb,
                max_stack_kb: raw.max_stack_kb,
                max_processes: raw.max_processes,
                max_open_files: raw.max_open_files,
                max_file_size_kb: raw.max_file_size_kb,
            })
    }
}


#[derive(Deserialize, Debug)]
pub struct Req {
    pub language: String,
    pub code: String
}

#[derive(Serialize, Debug)]
pub struct Resp {
    pub code: i32,
    pub output: String,
    pub std_log: String,
    pub time_ms: u128,
}

pub struct AppState {
    pub box_manager: Arc<BoxManager>,
}

impl AppState {
    pub fn new(box_manager: Arc<BoxManager>) -> Self {
        AppState { box_manager }
    }
}
