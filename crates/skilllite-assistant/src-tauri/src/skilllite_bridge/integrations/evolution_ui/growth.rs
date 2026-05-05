//! Life Pulse: whether periodic evolution should spawn (`evolution_growth_due`).

use std::collections::HashMap;

use crate::skilllite_bridge::chat::{merge_dotenv_with_chat_overrides, ChatConfigOverrides};
use crate::skilllite_bridge::paths::load_dotenv_for_child;

use super::config::{
    evolution_mode_from_workspace, growth_schedule_merged_for_workspace, EvolutionRunEnvGuard,
};

/// Desktop **Life Pulse** only: whether to spawn `skilllite evolution run`.
///
/// A9: periodic interval **or** weighted recent signals **or** raw unprocessed count **or** long-interval sweep.
/// When **only** the periodic arm is due and no proposals would be built, returns `false` (no subprocess,
/// no `evolution_run_outcome` spam). Signal/sweep arms still spawn even if proposals end up empty.
/// Merged workspace `.env` + UI overrides are applied for the preflight check so thresholds match the child.
///
/// `last_periodic_spawn_unix`: last time the **periodic** arm fired; updated when the periodic
/// condition is met. Initialized lazily on first check so the first periodic window starts then.
pub fn evolution_growth_due(
    workspace: &str,
    last_periodic_spawn_unix: &std::sync::Mutex<Option<i64>>,
    cfg: Option<&ChatConfigOverrides>,
) -> bool {
    let mode = evolution_mode_from_workspace(workspace);
    if mode.is_disabled() {
        return false;
    }
    let schedule_cfg = growth_schedule_merged_for_workspace(workspace, cfg);
    let chat_root = skilllite_core::paths::chat_root();
    let Ok(conn) = skilllite_evolution::feedback::open_evolution_db(&chat_root) else {
        return false;
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let mut g = last_periodic_spawn_unix
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let outcome =
        skilllite_evolution::growth_schedule::growth_due(&conn, now, &mut g, &schedule_cfg)
            .unwrap_or_default();

    if !outcome.due {
        return false;
    }
    if outcome.periodic_only {
        let dotenv = load_dotenv_for_child(workspace);
        let merged_vec = merge_dotenv_with_chat_overrides(dotenv, cfg);
        let merged_map: HashMap<String, String> = merged_vec.into_iter().collect();
        let _guard = EvolutionRunEnvGuard::push_from_merged(&merged_map);
        return skilllite_evolution::would_have_evolution_proposals(
            &conn,
            skilllite_evolution::EvolutionMode::from_env(),
            false,
        )
        .unwrap_or(true);
    }
    true
}
