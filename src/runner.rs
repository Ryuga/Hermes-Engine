use std::env;
use std::path::Path;
use std::process::Command;
use tokio::time::Instant;
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
    cmd.arg("--mem").arg(config.max_memory_kb.to_string());
    cmd.arg("--time").arg(config.max_cpu_time_sec.to_string());
    cmd.arg("--wall-time").arg(config.max_time_limit.to_string());
    cmd.arg("--extra-time").arg("2");
    cmd.arg("--stack").arg(config.max_stack_kb.to_string());
    cmd.arg("--open-files").arg(config.max_open_files.to_string());
    cmd.arg("--fsize").arg(config.max_file_size_kb.to_string());
    cmd.arg("--quota").arg("20000,2000"); // To be reviewed and adjusted
    cmd.arg("--core").arg("1024"); // 1MB for core dump to be reviewed and adjusted
    cmd.arg("--processes").arg(config.max_processes.to_string());

    // Environment
    // cmd.arg("--silent");
    cmd.arg("--env=PATH=/usr/bin:/bin");
    cmd.arg("--env=HOME=/tmp");

    // Metafile for job
    // TODO: read exit code
    let meta_path = format!("/tmp/isolate_{}.meta", isolate_box.id);
    cmd.arg("--meta").arg(&meta_path);

    cmd.arg("--run");
    cmd.arg("--");

    cmd.arg("/bin/sh");
    cmd.arg("-c");
    cmd.arg(&run_cmd);

    let out = cmd.output().map_err(|e| e.to_string())?;
    let time_ms = start.elapsed().as_millis();

    Ok((
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.code().unwrap_or(-1),
        time_ms,
    ))
}
