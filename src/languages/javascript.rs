use std::fs;
use std::path::Path;
use tracing::{warn};
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::config::models::{LangConfig, ReqMulti};

pub struct JavascriptHandler {
    config: &'static LangConfig,
}

impl JavascriptHandler {
    pub fn new(config: &'static LangConfig) -> Self {
        Self { config }
    }
}

impl LanguageHandler for JavascriptHandler {
    fn prepare(&self, work_dir: &Path, req: &ReqMulti) -> Result<PreparedProgram, String> {
        for file in &req.files {
            let item = work_dir.join(&file.name);

            if let Some(parent) = item.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            fs::write(item, &file.content).map_err(|e| e.to_string())?;
        }
        Ok(
            PreparedProgram {
                entry_file: work_dir.join(&req.entry_file),
                entry_name: req.entry_file.clone(),
                sources: None
            }
        )
    }
    fn compile_cmd(&self, _: &PreparedProgram) -> Vec<String> {
        warn!("No compilation required for javascript");
        vec![]
    }

    fn run_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let file_name = prepared.entry_file
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut cmd = Vec::with_capacity(2 + self.config.runtime_args.len());
        cmd.push(self.config.runtime_path.clone());
        cmd.extend(self.config.runtime_args.iter().cloned());
        cmd.push(file_name);
        cmd
    }
}
