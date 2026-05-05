//! User-authorized capability evolution: enqueue + background `evolution run`.

use crate::skilllite_bridge::paths::{find_project_root, load_dotenv_for_child};

pub fn authorize_capability_evolution(
    workspace: &str,
    tool_name: &str,
    outcome: &str,
    summary: &str,
    skilllite_path: &std::path::Path,
) -> Result<String, String> {
    let chat_root = skilllite_core::paths::chat_root();
    let conn =
        skilllite_evolution::feedback::open_evolution_db(&chat_root).map_err(|e| e.to_string())?;
    let proposal_id =
        skilllite_evolution::enqueue_user_capability_evolution(&conn, tool_name, outcome, summary)
            .map_err(|e| e.to_string())?;
    let _ = skilllite_evolution::log_evolution_event(
        &conn,
        &chat_root,
        "capability_evolution_authorized",
        tool_name,
        &format!("outcome={}, proposal_id={}", outcome, proposal_id),
        workspace,
    );
    // User-authorized proposals should be acted on promptly:
    // enqueue first (auditable), then trigger one immediate evolution run in background.
    let workspace_owned = workspace.to_string();
    let proposal_id_owned = proposal_id.clone();
    let skilllite_path_owned = skilllite_path.to_path_buf();
    let chat_root_for_log = chat_root.clone();
    std::thread::spawn(move || {
        let root = find_project_root(&workspace_owned);
        let mut cmd = std::process::Command::new(&skilllite_path_owned);
        crate::windows_spawn::hide_child_console(&mut cmd);
        cmd.arg("evolution")
            .arg("run")
            .arg("--json")
            .current_dir(&root)
            // Do NOT set SKILLLITE_WORKSPACE here: `agent-rpc` chat stores decisions under
            // `skilllite_core::paths::chat_root()` (default `~/.skilllite/chat` when workspace
            // env is unset). Forcing project root makes evolution open `<project>/chat`, a
            // different `feedback.sqlite` — empty decisions → "进化队列为空" and backlog mismatch.
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        for (k, v) in load_dotenv_for_child(&workspace_owned) {
            cmd.env(k, v);
        }
        // Must force this backlog row: generic `evolution run` only considers Passive/Active
        // proposals from build_evolution_proposals, not queued user-authorized capability rows.
        cmd.env(
            skilllite_core::config::env_keys::evolution::SKILLLITE_EVO_FORCE_PROPOSAL_ID,
            &proposal_id_owned,
        );
        let output = cmd.output();
        let status_note = match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut merged = stdout.trim().to_string();
                if !stderr.trim().is_empty() {
                    if !merged.is_empty() {
                        merged.push_str(" | ");
                    }
                    merged.push_str(stderr.trim());
                }
                if merged.is_empty() {
                    format!("trigger_exit={}", out.status)
                } else {
                    let mut clipped = merged;
                    if clipped.len() > 280 {
                        clipped.truncate(280);
                        clipped.push('…');
                    }
                    clipped
                }
            }
            Err(e) => format!("trigger_failed: {}", e),
        };
        if let Ok(conn) = skilllite_evolution::feedback::open_evolution_db(&chat_root_for_log) {
            let _ = skilllite_evolution::log_evolution_event(
                &conn,
                &chat_root_for_log,
                "capability_evolution_trigger_run",
                &proposal_id_owned,
                &status_note,
                &workspace_owned,
            );
        }
    });
    Ok(proposal_id)
}
