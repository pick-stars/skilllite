//! Evolution feedback DB snapshot for the assistant UI (`load_evolution_status`).

use serde::Serialize;

use crate::skilllite_bridge::chat::ChatConfigOverrides;

use super::config::{
    effective_evo_cooldown_hours, effective_evo_profile_key, evolution_mode_from_workspace,
    evolution_mode_labels, growth_schedule_merged_for_workspace,
};
use crate::skilllite_bridge::integrations::shared::existing_workspace_skills_root;

#[derive(Debug, Clone, Serialize)]
pub struct EvolutionLogEntryDto {
    pub ts: String,
    #[serde(rename = "event_type")]
    pub event_type: String,
    pub target_id: Option<String>,
    pub reason: Option<String>,
    /// `evolution_log.version`（常为本轮 txn，如 `evo_YYYYMMDD_HHMMSS`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvolutionStatusPayload {
    pub mode_key: String,
    pub mode_label: String,
    pub interval_secs: u64,
    pub decision_threshold: i64,
    /// Weighted sum over the latest `signal_window` meaningful unprocessed decisions (A9).
    pub weighted_signal_sum: i64,
    /// Env `SKILLLITE_EVO_TRIGGER_WEIGHTED_MIN` (after workspace merge for interval/threshold only).
    pub weighted_trigger_min: i64,
    pub signal_window: i64,
    /// `default` / `demo` / `conservative`（应用内覆盖优先于工作区 `.env`）。
    pub evo_profile_key: String,
    pub evo_cooldown_hours: f64,
    pub unprocessed_decisions: i64,
    pub last_run_ts: Option<String>,
    /// Latest material `evolution_run` timestamp (passive cooldown / A9 sweep clock).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_material_run_ts: Option<String>,
    pub judgement_label: Option<String>,
    pub judgement_reason: Option<String>,
    pub recent_events: Vec<EvolutionLogEntryDto>,
    pub pending_skill_count: usize,
    /// Read-only A9 arm breakdown (Life Pulse periodic anchor when available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a9: Option<skilllite_evolution::GrowthDueDiagnostics>,
    /// Passive gates + per-arm open state (same thresholds as `should_evolve_impl`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passive: Option<skilllite_evolution::PassiveScheduleDiagnostics>,
    /// Whether `build_evolution_proposals` would be non-empty for this workspace mode.
    pub would_have_evolution_proposals: bool,
    /// When [`Self::would_have_evolution_proposals`] is false: stable engine reason (English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_proposals_reason: Option<String>,
    pub db_error: Option<String>,
}

/// Evolution feedback DB + schedule hints for the assistant UI.
///
/// `periodic_anchor_unix`: last time the desktop **periodic** arm advanced (`LifePulseState`); pass
/// `None` when unavailable (e.g. first launch — periodic elapsed is reported from “now”).
pub fn load_evolution_status(
    workspace: &str,
    cfg: Option<ChatConfigOverrides>,
    periodic_anchor_unix: Option<i64>,
) -> EvolutionStatusPayload {
    let mode = evolution_mode_from_workspace(workspace);
    let (mode_key, mode_label) = evolution_mode_labels(&mode);

    let schedule_cfg = growth_schedule_merged_for_workspace(workspace, cfg.as_ref());
    let interval_secs = schedule_cfg.interval_secs;
    let decision_threshold = schedule_cfg.raw_unprocessed_threshold;
    let weighted_trigger_min = schedule_cfg.weighted_min;
    let signal_window = schedule_cfg.signal_window;
    let evo_profile_key = effective_evo_profile_key(workspace, cfg.as_ref()).to_string();
    let evo_cooldown_hours = effective_evo_cooldown_hours(workspace, cfg.as_ref());

    let chat_root = skilllite_core::paths::chat_root();
    let mut pending_skill_count = 0;
    if let Some(skills_root) = existing_workspace_skills_root(workspace) {
        pending_skill_count =
            skilllite_evolution::skill_synth::list_pending_skills_with_review(&skills_root).len();
    }

    let mut db_error = None;
    let mut unprocessed_decisions = 0i64;
    let mut weighted_signal_sum = 0i64;
    let mut recent_events = Vec::new();
    let mut last_run_ts = None;
    let mut last_material_run_ts = None;
    let mut judgement_label = None;
    let mut judgement_reason = None;
    let mut a9 = None;
    let mut passive = None;
    let mut would_have_evolution_proposals = false;
    let mut empty_proposals_reason: Option<String> = None;

    match skilllite_evolution::feedback::open_evolution_db(&chat_root) {
        Ok(conn) => {
            if let Ok(c) = skilllite_evolution::feedback::count_unprocessed_decisions(&conn) {
                unprocessed_decisions = c;
            }
            if let Ok(w) = skilllite_evolution::growth_schedule::weighted_unprocessed_signal_sum(
                &conn,
                signal_window,
            ) {
                weighted_signal_sum = w;
            }
            if let Ok(Some(summary)) = skilllite_evolution::feedback::build_latest_judgement(&conn)
            {
                judgement_label = Some(summary.judgement.label_zh().to_string());
                judgement_reason = Some(summary.reason);
            }
            // Last UI "attempt" timestamp: material run or no-output run (constants are fixed literals).
            let last_attempt_sql = format!(
                "SELECT ts FROM evolution_log WHERE type IN ('{}', '{}') ORDER BY ts DESC LIMIT 1",
                skilllite_evolution::feedback::EVOLUTION_LOG_TYPE_RUN_MATERIAL,
                skilllite_evolution::feedback::EVOLUTION_LOG_TYPE_RUN_NOOP,
            );
            if let Ok(mut stmt) = conn.prepare(&last_attempt_sql) {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        last_run_ts = row.get(0).ok();
                    }
                }
            }
            let last_mat_sql = format!(
                "SELECT ts FROM evolution_log WHERE type = '{}' ORDER BY ts DESC LIMIT 1",
                skilllite_evolution::feedback::EVOLUTION_LOG_TYPE_RUN_MATERIAL,
            );
            if let Ok(mut stmt) = conn.prepare(&last_mat_sql) {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        last_material_run_ts = row.get(0).ok();
                    }
                }
            }

            let now_unix = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            a9 = skilllite_evolution::inspect_growth_due(
                &conn,
                now_unix,
                periodic_anchor_unix,
                &schedule_cfg,
            )
            .ok();
            passive = skilllite_evolution::passive_schedule_diagnostics(&conn, &mode).ok();
            would_have_evolution_proposals =
                skilllite_evolution::would_have_evolution_proposals(&conn, mode.clone(), false)
                    .unwrap_or(false);
            if !would_have_evolution_proposals {
                empty_proposals_reason =
                    skilllite_evolution::describe_empty_evolution_proposals(&conn, &mode, false)
                        .ok()
                        .map(str::to_string);
            }

            if let Ok(mut stmt) = conn.prepare(
                "SELECT ts, type, target_id, reason, version FROM evolution_log ORDER BY ts DESC LIMIT 25",
            ) {
                if let Ok(rows) = stmt.query_map([], |row| {
                    let version: Option<String> = row.get(4)?;
                    let txn_id = version
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.trim().to_string());
                    Ok(EvolutionLogEntryDto {
                        ts: row.get(0)?,
                        event_type: row.get(1)?,
                        target_id: row.get::<_, Option<String>>(2)?,
                        reason: row.get::<_, Option<String>>(3)?,
                        txn_id,
                    })
                }) {
                    recent_events.extend(rows.flatten());
                }
            }
        }
        Err(e) => {
            db_error = Some(format!("无法打开进化数据库: {}", e));
        }
    }

    EvolutionStatusPayload {
        mode_key: mode_key.to_string(),
        mode_label: mode_label.to_string(),
        interval_secs,
        decision_threshold,
        weighted_signal_sum,
        weighted_trigger_min,
        signal_window,
        evo_profile_key,
        evo_cooldown_hours,
        unprocessed_decisions,
        last_run_ts,
        last_material_run_ts,
        judgement_label,
        judgement_reason,
        recent_events,
        pending_skill_count,
        a9,
        passive,
        would_have_evolution_proposals,
        empty_proposals_reason,
        db_error,
    }
}
