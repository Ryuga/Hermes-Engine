use tempfile::tempdir;
use tracing::debug;
use crate::languages::get_handler;
use crate::loader::get_lang_config;
use crate::models::{Req, Resp};
use crate::runner::safe_execute;
use crate::workers::IsolateBox;


pub fn execute_code(isolate_box: &IsolateBox, req: Req, passed_token: Option<String>) -> Result<Resp, String>{
    let lang_config = get_lang_config(req.language.as_str());

    if lang_config.authenticate {
        let secret = std::env::var("API_TOKEN")
            .map_err(|_| "Server configuration error".to_string())?;

        let is_authorized = passed_token
            .map(|t| t == format!("Bearer {}", secret))
            .unwrap_or(false);

        if !is_authorized {
            return Ok(Resp {
                output: String::new(),
                std_log: "Auth Error: This language requires a valid API key.".to_string(),
                code: 401,
                time_ms: 0,
            });
        }
    }

    let handler = get_handler(req.language.as_str(), lang_config.clone());

    let work_dir = &isolate_box.path;

    debug!("Preparing programing for execution");
    let program = match handler.prepare(work_dir, &req.code) {
        Ok(p) => p,
        Err(e) => {
            return Ok(Resp{
                output: "Program preparation failed".to_string(),
                std_log: e,
                code: 1,
                time_ms: 0
            });
        }
    };

    debug!("Finalizing execution command");
    let exe_cmd = if lang_config.compile {
        format!(
            "{} && {}",
            handler.compile_cmd(&program).join(" "),
            handler.run_cmd(&program).join(" ")
        )
    }
    else {
        handler.run_cmd(&program).join(" ")
    };

    debug!("Executing command: {}", exe_cmd);
    let (output, std_log, code, time_ms) =
        safe_execute(&isolate_box, lang_config.clone(), exe_cmd)?;

    Ok(Resp{output, std_log, code, time_ms})
}
