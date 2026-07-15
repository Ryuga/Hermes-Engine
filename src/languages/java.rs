use std::fs;
use std::path::Path;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::instrument;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::config::models::{LangConfig, ReqMulti};

pub struct JavaHandler {
    config: &'static LangConfig,
}

static IMPORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*import\s+([a-zA-Z0-9_.]+);").unwrap()
});

static CLASS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*public\s+(?:\w+\s+)*class\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap()
});


impl JavaHandler {
    pub fn new(config: &'static LangConfig) -> Self {
        Self { config }
    }

    fn check_for_external_imports_and_packages(code: &str) -> Result<(), String> {
        if code.contains("package ") {
            return Err("Preparation Error: 'package' declarations are not allowed.".to_string());
        }

        for matches in IMPORT_RE.captures_iter(code) {
            let import = &matches[1];
            if !(import.starts_with("java.")
                || import.starts_with("javax.")
                || import.starts_with("jdk."))
            {
                return Err(
                    format!("Preparation Error: external import '{}' is not allowed.", import)
                );
            }
        }
        Ok(())
    }

    fn extract_main_class_name(code: &str) -> Result<String, String> {

        let res = CLASS_RE.captures(&code)
            .ok_or("Compilation Error: Java programs must declare a public class.")?;
        Ok(res[1].to_string())
    }
}

impl LanguageHandler for JavaHandler {

    #[instrument(level = "debug", skip(self))]
    fn prepare(&self, work_dir: &Path, req: &ReqMulti) -> Result<PreparedProgram, String> {

        let mut main_class_name: Option<String> = None;

        for file in &req.files {
            Self::check_for_external_imports_and_packages(&file.content)?;

            let is_entry = file.name == req.entry_file;

            let final_name = if is_entry {
                let name = Self::extract_main_class_name(&file.content)?;
                main_class_name = Some(name.clone());
                format!("{}.java", name)
            } else {
                file.name.clone()
            };

            let file_path = work_dir.join(final_name);

            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            fs::write(&file_path, &file.content).map_err(|e| e.to_string())?;
        }
        let entry_name = main_class_name
            .ok_or("Preparation Error: Entry file not found or invalid.")?;

        Ok(
            PreparedProgram {
                entry_file: work_dir.join(format!("{}.java", entry_name)),
                entry_name,
                sources: None
            }
        )
    }

    fn compile_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let mut cmd = vec![self.config.compiler_path.clone()];
        cmd.extend(self.config.compiler_args.clone());
        cmd.push(format!("{}.java", prepared.entry_name));
        cmd
    }

    fn run_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let mut cmd = vec![self.config.runtime_path.clone()];
        cmd.extend(self.config.runtime_args.clone());
        cmd.push(prepared.entry_name.clone());
        cmd
    }
}
