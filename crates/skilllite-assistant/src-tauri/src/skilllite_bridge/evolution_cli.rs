//! Spawn `skilllite` CLI with workspace env merge (L2 bridge for desktop evolution UI).

use std::path::Path;
use std::process::Command;

use serde::de::DeserializeOwned;

use super::chat::{merge_dotenv_with_chat_overrides, ChatConfigOverrides};
use super::paths::{find_project_root, load_dotenv_for_child};

pub fn spawn_skilllite(
    skilllite_path: &Path,
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
    args: &[&str],
) -> Result<std::process::Output, String> {
    let root = find_project_root(workspace);
    let mut cmd = Command::new(skilllite_path);
    crate::windows_spawn::hide_child_console(&mut cmd);
    cmd.args(args);
    let env_pairs = merge_dotenv_with_chat_overrides(load_dotenv_for_child(workspace), cfg);
    for (k, v) in env_pairs {
        cmd.env(k, v);
    }
    cmd.current_dir(&root)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    cmd.output()
        .map_err(|e| format!("spawn skilllite {}: {}", args.join(" "), e))
}

pub fn spawn_skilllite_json<T: DeserializeOwned>(
    skilllite_path: &Path,
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
    args: &[&str],
) -> Result<T, String> {
    let output = spawn_skilllite(skilllite_path, workspace, cfg, args)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "skilllite {} failed (exit {:?}): {}",
            args.join(" "),
            output.status.code(),
            stderr.trim()
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| {
        format!(
            "parse skilllite {} JSON: {} (stdout len={})",
            args.join(" "),
            e,
            output.stdout.len()
        )
    })
}
