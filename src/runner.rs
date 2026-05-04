use std::env;
use std::path::Path;
use std::process::Command;
use tokio::time::Instant;
use tracing::debug;
use tracing_subscriber::fmt::format;
use crate::models::LangConfig;
use crate::workers::IsolateBox;

pub fn safe_execute(isolate_box: &IsolateBox,
                    config: LangConfig,
                    run_cmd: String
) -> Result<(String, String, i32, u128), String> {
    let start = Instant::now();

    let mut cmd = Command::new("isolate");
    // Box config
    cmd.arg("--box-id").arg(&isolate_box.id.to_string());
    cmd.arg("--cg");

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
    cmd.arg(format!("--processes={}", config.max_processes.to_string()));

    // Environment
    // cmd.arg("--silent");
    cmd.arg("--env=PATH=/usr/bin:/bin");
    cmd.arg("--env=HOME=/tmp");

    // Metafile for job
    // TODO: read exit code
    let meta_path = format!("/tmp/isolate_{}.meta", isolate_box.id);
    cmd.arg(format!("--meta={}", &meta_path));

    cmd.arg("--run");
    cmd.arg("--");

    cmd.arg("/bin/sh");
    cmd.arg("-c");
    cmd.arg(&run_cmd);

    debug!("Executing: {:?}", cmd);

    let out = cmd.output().map_err(|e| e.to_string())?;
    let time_ms = start.elapsed().as_millis();

    Ok((
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.code().unwrap_or(-1),
        time_ms,
    ))
}
