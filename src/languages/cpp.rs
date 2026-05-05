use std::fs;
use std::path::Path;
use regex::Regex;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::models::LangConfig;

pub struct CppHandler {
    config: &'static LangConfig,
}

impl CppHandler {
    pub fn new(config: &'static LangConfig) -> Self {
        Self { config }
    }

    fn check_for_external_includes(code: &str) -> Result<(), String> {
        let local_include_re = Regex::new(r#"(?m)^\s*#\s*include\s*"([^"]+)""#).unwrap();

        if let Some(caps) = local_include_re.captures(code) {
            return Err(format!(
                "Compilation Error: Local include '\"{}\"' is not allowed. \
                Please use only standard headers.",
                &caps[1]
            ));
        }

        let path_traversal_re = Regex::new(r#"(?m)^\s*#\s*include\s*<.*(\.\.|/).*>"#).unwrap();
        if path_traversal_re.is_match(code) {
            return Err("Compilation Error: Path traversal in includes is forbidden.".to_string());
        }

        Ok(())
    }
}

impl LanguageHandler for CppHandler {
    fn prepare(&self, work_dir: &Path, code: &str) -> Result<PreparedProgram, String> {
        Self::check_for_external_includes(code)?;

        let file_path = work_dir.join(&self.config.source);
        fs::write(&file_path, code).map_err(|e| e.to_string())?;

        let bin_name = Path::new(&self.config.source)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("solution")
            .to_string();

        Ok(PreparedProgram {
            entry_file: file_path,
            entry_name: bin_name,
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