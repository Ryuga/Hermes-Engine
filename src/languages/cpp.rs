use std::fs;
use std::path::Path;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::config::models::{LangConfig, ReqMulti};


static EXTERNAL_REF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)#include\s*[<"]\s*(/|\.\.).*[>"]"#).unwrap()
});

pub struct CppHandler {
    config: &'static LangConfig,
}

impl CppHandler {
    pub fn new(config: &'static LangConfig) -> Self {
        Self { config }
    }

    fn check_for_security_violations(code: &str) -> Result<(), String> {
        if EXTERNAL_REF_RE.is_match(code) {
            return Err("Compilation Error: Absolute paths or path traversal in includes is forbidden.".to_string());
        }

        Ok(())
    }
}

impl LanguageHandler for CppHandler {
    fn prepare(&self, work_dir: &Path, req: &ReqMulti) -> Result<PreparedProgram, String> {

        let mut entry_file_path = None;

        for file in &req.files {
            Self::check_for_security_violations(&file.content)?;

            let item = work_dir.join(&file.name);

            if let Some(parent) = item.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            fs::write(&item, &file.content).map_err(|e| e.to_string())?;

            if file.name == req.entry_file {
                entry_file_path = Some(item);
            }
        }

        let entry_file = entry_file_path.ok_or("Entry file not found")?;

        Ok(PreparedProgram {
            entry_file,
            entry_name: "solution".to_string(),
        })
    }

    fn compile_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let mut cmd = vec![self.config.compiler_path.clone()];
        cmd.extend(self.config.compiler_args.clone());

        let source_name = prepared.entry_file.file_name().unwrap().to_str().unwrap();

        cmd.push(source_name.to_string());
        cmd.push("-o".to_string());
        cmd.push(prepared.entry_name.clone());

        cmd
    }

    fn run_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let mut cmd = Vec::new();

        cmd.push(format!("./{}", prepared.entry_name));

        cmd
    }
}