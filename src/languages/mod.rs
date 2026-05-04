pub mod python;
mod javascript;
mod java;
mod cpp;

use std::path::{Path, PathBuf};
use crate::languages::cpp::CppHandler;
use crate::languages::java::JavaHandler;
use crate::languages::javascript::JavascriptHandler;
use crate::languages::python::PythonHandler;
use crate::loader::get_lang_config;
use crate::models::LangConfig;

pub struct PreparedProgram {
    pub entry_file: PathBuf,
    pub entry_name: String,
}

pub trait LanguageHandler {
    fn prepare(&self, work_dir: &Path, code: &str) -> Result<PreparedProgram, String>;
    fn compile_cmd(&self, prepared: &PreparedProgram) -> Vec<String>;
    fn run_cmd(&self, prepared: &PreparedProgram) -> Vec<String>;
}

pub fn get_handler(lang: &str) -> Box<dyn LanguageHandler> {
    let config = get_lang_config(lang);

    match lang {
        "python" => Box::new(PythonHandler::new(config)),
        "javascript" => Box::new(JavascriptHandler::new(config)),
        "java" => Box::new(JavaHandler::new(config)),
        "cpp" => Box::new(CppHandler::new(config)),
        _ => panic!("Unsupported language"),
    }
}
