use std::env;
use std::process::Command;
use once_cell::sync::Lazy;
use tokio::time::Instant;
use tracing::debug;
use crate::models::LangConfig;
use crate::workers::IsolateBox;

static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("RUST_LOG") {
        Ok(val) => {
            let v = val.to_lowercase();
            v == "debug" || v == "full"
        }
        Err(_) => false,
    }
});

pub fn safe_execute(isolate_box: &IsolateBox,
                    config: &LangConfig,
                    run_args: &[String]
) -> Result<(String, String, i32, u128), String> {
    let start = Instant::now();

    let mut cmd = Command::new("isolate");
    // Box config
    cmd.arg("--box-id").arg(&isolate_box.id.to_string());
    cmd.arg("--cg");

    cmd.args(&config.isolate_args);

    // Resource limit enforcement
    cmd.arg(format!("--mem={}", config.max_memory_kb));
    cmd.arg(format!("--time={}", config.max_cpu_time_sec));
    cmd.arg(format!("--wall-time={}", config.max_time_limit));
    cmd.arg(format!("--extra-time={}", "2")); // To be adjusted per lang
    cmd.arg(format!("--stack={}", config.max_stack_kb));
    cmd.arg(format!("--open-files={}", config.max_open_files));
    cmd.arg(format!("--fsize={}", config.max_file_size_kb));
    cmd.arg(format!("--quota={}", "20000,2000")); // To be reviewed and adjusted
    cmd.arg(format!("--core={}", "1024")); // 1MB for core dump to be reviewed and adjusted
    cmd.arg(format!("--processes={}", config.max_processes));

    // Environment
    if !*DEBUG {
        cmd.arg("--silent");
    }
    cmd.arg("--env=PATH=/usr/bin:/bin");
    cmd.arg("--env=HOME=/tmp");

    // Metafile for job
    // TODO: read exit code
    let meta_path = format!("/tmp/isolate_{}.meta", isolate_box.id);
    cmd.arg(format!("--meta={}", &meta_path));

    cmd.arg("--run");
    cmd.arg("--");

    cmd.args(run_args);

    debug!(?cmd, "Executing isolate command");

    let out = cmd.output().map_err(|e| e.to_string())?;
    let time_ms = start.elapsed().as_millis();

    Ok((
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.code().unwrap_or(-1),
        time_ms,
    ))
}
