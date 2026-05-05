//! Workspace / UI merged evolution config and env guard for in-process `run_evolution`.

use crate::skilllite_bridge::chat::ChatConfigOverrides;
use crate::skilllite_bridge::paths::load_dotenv_for_child;

pub(super) fn workspace_env_lookup(workspace: &str, key: &str) -> Option<String> {
    load_dotenv_for_child(workspace)
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .or_else(|| std::env::var(key).ok())
}

pub(super) fn is_evolution_runtime_env_key(k: &str) -> bool {
    use skilllite_core::config::env_keys::evolution as evo;
    k == evo::SKILLLITE_EVOLUTION
        || k == evo::SKILLLITE_MAX_EVOLUTIONS_PER_DAY
        || k.starts_with("SKILLLITE_EVO") // SKIP-ENV-AUDIT: prefix-match, not a single var
        || k.starts_with("SKILLLITE_EVOLUTION_") // SKIP-ENV-AUDIT: prefix-match, not a single var
}

/// 将合并后的子进程环境变量中、与进化运行时相关的键同步到当前进程，供 `run_evolution` 内 `from_env()` 读取。
pub(super) struct EvolutionRunEnvGuard {
    prev: Vec<(String, Option<String>)>,
}

impl EvolutionRunEnvGuard {
    pub(super) fn push_from_merged(m: &std::collections::HashMap<String, String>) -> Self {
        let mut prev = Vec::new();
        for (k, v) in m.iter() {
            if !is_evolution_runtime_env_key(k) {
                continue;
            }
            prev.push((k.clone(), std::env::var(k).ok()));
            std::env::set_var(k, v);
        }
        Self { prev }
    }
}

impl Drop for EvolutionRunEnvGuard {
    fn drop(&mut self) {
        for (k, ov) in self.prev.drain(..) {
            match ov {
                Some(val) => std::env::set_var(&k, val),
                None => std::env::remove_var(&k),
            }
        }
    }
}

pub(super) fn effective_evolution_interval_secs(
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
) -> u64 {
    use skilllite_core::config::env_keys::evolution as evo_env;
    if let Some(c) = cfg {
        if let Some(n) = c.evolution_interval_secs.filter(|&n| n > 0) {
            return n;
        }
    }
    workspace_env_lookup(workspace, evo_env::SKILLLITE_EVOLUTION_INTERVAL_SECS)
        .and_then(|v| v.parse().ok())
        .unwrap_or(600)
}

pub(super) fn effective_evolution_decision_threshold(
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
) -> i64 {
    use skilllite_core::config::env_keys::evolution as evo_env;
    if let Some(c) = cfg {
        if let Some(n) = c.evolution_decision_threshold.filter(|&n| n > 0) {
            return i64::from(n);
        }
    }
    workspace_env_lookup(workspace, evo_env::SKILLLITE_EVOLUTION_DECISION_THRESHOLD)
        .and_then(|v| v.parse().ok())
        .unwrap_or(10)
}

pub(super) fn effective_evo_profile_key(
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
) -> &'static str {
    use skilllite_core::config::env_keys::evolution as evo_env;
    if let Some(c) = cfg {
        if let Some(ref p) = c.evo_profile {
            let t = p.trim();
            if t == "demo" {
                return "demo";
            }
            if t == "conservative" {
                return "conservative";
            }
        }
    }
    match workspace_env_lookup(workspace, evo_env::SKILLLITE_EVO_PROFILE).as_deref() {
        Some("demo") => "demo",
        Some("conservative") => "conservative",
        _ => "default",
    }
}

pub(super) fn effective_evo_cooldown_hours(
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
) -> f64 {
    use skilllite_core::config::env_keys::evolution as evo_env;
    if let Some(c) = cfg {
        if let Some(h) = c.evo_cooldown_hours.filter(|h| h.is_finite() && *h >= 0.0) {
            return h;
        }
    }
    workspace_env_lookup(workspace, evo_env::SKILLLITE_EVO_COOLDOWN_HOURS)
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| skilllite_evolution::EvolutionThresholds::default().cooldown_hours)
}

/// Merged A9 growth config: workspace `.env` + optional UI overrides for interval / raw threshold.
pub(super) fn growth_schedule_merged_for_workspace(
    workspace: &str,
    cfg: Option<&ChatConfigOverrides>,
) -> skilllite_evolution::growth_schedule::GrowthScheduleConfig {
    let mut c = skilllite_evolution::growth_schedule::GrowthScheduleConfig::from_env();
    c.interval_secs = effective_evolution_interval_secs(workspace, cfg);
    c.raw_unprocessed_threshold = effective_evolution_decision_threshold(workspace, cfg);
    c
}

pub(super) fn evolution_mode_from_workspace(workspace: &str) -> skilllite_evolution::EvolutionMode {
    use skilllite_core::config::env_keys::evolution as evo_env;
    match workspace_env_lookup(workspace, evo_env::SKILLLITE_EVOLUTION).as_deref() {
        None | Some("1") | Some("true") | Some("") => skilllite_evolution::EvolutionMode::All,
        Some("0") | Some("false") => skilllite_evolution::EvolutionMode::Disabled,
        Some("prompts") => skilllite_evolution::EvolutionMode::PromptsOnly,
        Some("memory") => skilllite_evolution::EvolutionMode::MemoryOnly,
        Some("skills") => skilllite_evolution::EvolutionMode::SkillsOnly,
        _ => skilllite_evolution::EvolutionMode::All,
    }
}

pub(super) fn evolution_mode_labels(
    mode: &skilllite_evolution::EvolutionMode,
) -> (&'static str, &'static str) {
    match mode {
        skilllite_evolution::EvolutionMode::All => ("all", "全部启用"),
        skilllite_evolution::EvolutionMode::PromptsOnly => ("prompts", "仅 Prompts"),
        skilllite_evolution::EvolutionMode::MemoryOnly => ("memory", "仅 Memory"),
        skilllite_evolution::EvolutionMode::SkillsOnly => ("skills", "仅 Skills"),
        skilllite_evolution::EvolutionMode::Disabled => ("disabled", "已禁用"),
    }
}
