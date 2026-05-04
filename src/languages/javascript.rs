use std::fs;
use std::path::Path;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::models::LangConfig;

pub struct JavascriptHandler {
    config: &'static LangConfig,
}

impl JavascriptHandler {
    pub fn new(config: &'static LangConfig) -> Self {
        Self { config }
    }
}

impl LanguageHandler for JavascriptHandler {
    fn prepare(&self, work_dir: &Path, code: &str) -> Result<PreparedProgram, String> {
        let file = work_dir.join(&self.config.source);
        fs::write(&file, code).map_err(|e| e.to_string())?;
        Ok(
            PreparedProgram {
                entry_file: file,
                entry_name: self.config.source.clone(),
            }
        )
    }
    fn compile_cmd(&self, _: &PreparedProgram) -> Vec<String> {
        println!("Ignoring compilation for javascript...");
        unimplemented!()
    }

    fn run_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let file_name = prepared.entry_file.
            file_name().unwrap().to_string_lossy().to_string();

        let mut cmd = vec![self.config.runtime_path.clone()];
        cmd.extend(self.config.runtime_args.clone());
        cmd.push(file_name);
        cmd
    }
}
