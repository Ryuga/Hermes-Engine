use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

use super::utils::string_or_int;
use crate::api::utils::Validate;

fn default_vector() -> Vec<String> { vec![] }
fn default_compile() -> bool { false }
fn default_authenticate() -> bool { false }
fn default_time_limit() -> String { 2.to_string() }
fn default_cpu_time_sec() -> String { 2.to_string() }
fn default_memory_kb() -> String { (256 * 1024).to_string() }
fn default_stack_kb() -> String { (64 * 1024).to_string() }
fn default_processes() -> String { 16.to_string() }
fn default_open_files() -> String { 64.to_string() }
fn default_file_size_kb() -> String { 1024.to_string() }


#[derive(Clone, Debug)]
pub struct LangConfig {
    pub source: String,

    pub compile: bool,

    pub authenticate: bool,

    pub compiler_path: String,
    pub compiler_args: Vec<String>,

    pub runtime_path: String,
    pub runtime_args: Vec<String>,
    pub isolate_args: Vec<String>,

    pub max_time_limit: String,
    pub max_cpu_time_sec: String,
    pub max_memory_kb: String,
    pub max_stack_kb: String,
    pub max_processes: String,
    pub max_open_files: String,
    pub max_file_size_kb: String,
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

    #[serde(default = "default_vector")]
    pub isolate_args: Vec<String>,

    #[serde(default = "default_time_limit",deserialize_with = "string_or_int")]
    pub max_time_limit: String,

    #[serde(default = "default_cpu_time_sec",deserialize_with = "string_or_int")]
    pub max_cpu_time_sec: String,

    #[serde(default = "default_memory_kb",deserialize_with = "string_or_int")]
    pub max_memory_kb: String,

    #[serde(default = "default_stack_kb",deserialize_with = "string_or_int")]
    pub max_stack_kb: String,

    #[serde(default = "default_processes",deserialize_with = "string_or_int")]
    pub max_processes: String,

    #[serde(default = "default_open_files",deserialize_with = "string_or_int")]
    pub max_open_files: String,

    #[serde(default = "default_file_size_kb",deserialize_with = "string_or_int")]
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
                isolate_args: raw.isolate_args,
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

#[derive(Deserialize, Debug, Clone)]
pub struct File{
    pub name: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ReqMulti {
    pub language: String,
    pub files: Vec<File>,
    pub entry_file: String,
}

impl Validate for ReqMulti {
    fn validate(&self) -> Result<(), String> {

        let mut entry_file_found = false;

        if self.files.is_empty() {
            return Err("Preparation Error: No files provided".into());
        }

        for file in &self.files {
            if file.name.contains("..") || file.name.starts_with('/') {
                return Err(
                    format!("Security Violation: Invalid path in filename '{}'", file.name)
                );
            }
            if file.name == self.entry_file {
                entry_file_found = true;
            }
        }

        if !entry_file_found {
            return Err(
                format!(
                    "Preparation Error: Entry file '{}' not found in file list",
                    self.entry_file
                )
            );
        }

        Ok(())
    }
}
