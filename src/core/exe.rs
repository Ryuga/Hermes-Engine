use tracing::instrument;
use crate::languages::get_handler;
use crate::core::runner::safe_execute;
use crate::core::workers::{BoxManager, EphemeralBox, PersistentBox};
use crate::config::models::{ReqMulti, Resp};
use crate::config::utils::get_lang_config;

#[instrument(level = "debug", skip(passed_token))]
pub fn execute_code(compiler_pool: &BoxManager<PersistentBox>,
                    executor_pool: &BoxManager<EphemeralBox>,
                    req: ReqMulti,
                    passed_token: Option<String>) -> Result<Resp, String>{

    let lang_config = match get_lang_config(&req.language) {
        Ok(config) => config,
        Err(e) => return Ok(
            Resp {
                output: "Unsupported runtime".to_string(),
                std_log: e,
                time_ms: 0,
                code: 1
            }
        )
    };

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

    let executor_box = executor_pool.acquire();

    let handler = get_handler(&req.language).map_err(|e| e)?;

    let work_dir = &executor_box.path;

    let program = match handler.prepare(work_dir, &req) {
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

    if lang_config.compile {
        let compile_args = handler.compile_cmd(&program);
        let (out, log, code, _) = safe_execute(&executor_box, lang_config, &compile_args)?;
        if code != 0 {
            return Ok(Resp {
                output: out,
                std_log: format!("Compilation Error:\n{}", log),
                code,
                time_ms: 0,
            });
        }
    }

    let run_args = handler.run_cmd(&program);

    let (output, std_log, code, time_ms) =
        safe_execute(&executor_box, &lang_config, &run_args)?;

    executor_pool.release(executor_box);

    Ok(Resp{output, std_log, code, time_ms})
}
