//! In-process `run_evolution` for manual UI trigger.

use crate::skilllite_bridge::chat::{merge_dotenv_with_chat_overrides, ChatConfigOverrides};
use crate::skilllite_bridge::integrations::shared::existing_workspace_skills_root;
use crate::skilllite_bridge::paths::load_dotenv_for_child;

use super::config::EvolutionRunEnvGuard;

pub fn trigger_evolution_run(
    workspace: &str,
    proposal_id: Option<&str>,
    _skilllite_path: &std::path::Path,
    overrides: Option<ChatConfigOverrides>,
) -> Result<String, String> {
    fn env_first_non_empty(
        vars: &std::collections::HashMap<String, String>,
        keys: &[&str],
    ) -> Option<String> {
        for k in keys {
            if let Some(v) = vars.get(*k) {
                let t = v.trim();
                if !t.is_empty() {
                    return Some(t.to_string());
                }
            }
        }
        None
    }

    use skilllite_core::config::env_keys::llm as llm_keys;
    let env_map: std::collections::HashMap<String, String> =
        load_dotenv_for_child(workspace).into_iter().collect();
    let api_base_keys: Vec<&str> = std::iter::once(llm_keys::API_BASE)
        .chain(llm_keys::API_BASE_ALIASES.iter().copied())
        .collect();
    let api_key_keys: Vec<&str> = std::iter::once(llm_keys::API_KEY)
        .chain(llm_keys::API_KEY_ALIASES.iter().copied())
        .collect();
    let model_keys: Vec<&str> = std::iter::once(llm_keys::MODEL)
        .chain(llm_keys::MODEL_ALIASES.iter().copied())
        .collect();
    let mut api_base = env_first_non_empty(&env_map, &api_base_keys)
        .or_else(|| skilllite_core::config::LlmConfig::try_from_env().map(|c| c.api_base))
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
    let mut api_key = env_first_non_empty(&env_map, &api_key_keys)
        .or_else(|| skilllite_core::config::LlmConfig::try_from_env().map(|c| c.api_key))
        .unwrap_or_default();
    let mut model = env_first_non_empty(&env_map, &model_keys)
        .or_else(|| skilllite_core::config::LlmConfig::try_from_env().map(|c| c.model))
        .unwrap_or_else(|| "gpt-4o".to_string());

    // Match chat: keys/base/model from Settings are not written to process env; merge here so
    // manual evolution trigger works when the user only configured the assistant UI.
    if let Some(ref cfg) = overrides {
        if let Some(ref b) = cfg.api_base {
            if !b.trim().is_empty() {
                api_base = b.clone();
            }
        }
        if let Some(ref k) = cfg.api_key {
            if !k.trim().is_empty() {
                api_key = k.clone();
            }
        }
        if let Some(ref m) = cfg.model {
            if !m.trim().is_empty() {
                model = m.clone();
            }
        }
    }

    if api_key.trim().is_empty() {
        return Err(
            "执行 evolution run 失败: 缺少 API key（请配置 SKILLLITE_API_KEY 或 OPENAI_API_KEY）"
                .to_string(),
        );
    }

    let dotenv = load_dotenv_for_child(workspace);
    let merged_vec = merge_dotenv_with_chat_overrides(dotenv, overrides.as_ref());
    let merged_map: std::collections::HashMap<String, String> = merged_vec.into_iter().collect();
    let _evo_env_guard = EvolutionRunEnvGuard::push_from_merged(&merged_map);

    let llm = skilllite_agent::llm::LlmClient::new(&api_base, &api_key)
        .map_err(|e| format!("执行 evolution run 失败: 初始化 LLM 客户端失败: {}", e))?;
    let adapter = skilllite_agent::evolution::EvolutionLlmAdapter { llm: &llm };
    let skills_root = existing_workspace_skills_root(workspace);

    let force_key = skilllite_core::config::env_keys::evolution::SKILLLITE_EVO_FORCE_PROPOSAL_ID;
    let prev_force = std::env::var(force_key).ok();
    match proposal_id {
        Some(pid) => std::env::set_var(force_key, pid),
        None => std::env::remove_var(force_key),
    }
    let run_result = tokio::runtime::Runtime::new()
        .map_err(|e| format!("执行 evolution run 失败: runtime 初始化失败: {}", e))?
        .block_on(skilllite_evolution::run_evolution(
            &skilllite_core::paths::chat_root(),
            skills_root.as_deref(),
            &adapter,
            &api_base,
            &api_key,
            &model,
            true,
        ));
    match prev_force {
        Some(v) => std::env::set_var(force_key, v),
        None => std::env::remove_var(force_key),
    }

    let response = match run_result {
        Ok(skilllite_evolution::EvolutionRunResult::Completed(Some(txn_id))) => {
            let conn = skilllite_evolution::feedback::open_evolution_db(
                &skilllite_core::paths::chat_root(),
            )
            .map_err(|e| format!("执行 evolution run 失败: 打开进化数据库失败: {}", e))?;
            let changes = skilllite_evolution::query_changes_by_txn(&conn, &txn_id);
            let summary: Vec<String> = skilllite_evolution::format_evolution_changes(&changes);
            if summary.is_empty() {
                format!("Evolution completed (txn={})", txn_id)
            } else {
                summary.join("\n")
            }
        }
        Ok(skilllite_evolution::EvolutionRunResult::SkippedBusy) => {
            "Evolution skipped: another run in progress".to_string()
        }
        Ok(skilllite_evolution::EvolutionRunResult::NoScope)
        | Ok(skilllite_evolution::EvolutionRunResult::Completed(None)) => {
            let mut hint = String::from("Evolution: nothing to evolve");
            if let Ok(conn) = skilllite_evolution::feedback::open_evolution_db(
                &skilllite_core::paths::chat_root(),
            ) {
                if let Ok((total, with_desc)) =
                    skilllite_evolution::feedback::count_decisions_with_task_desc(&conn)
                {
                    if total > 0 && with_desc == 0 {
                        hint.push_str("\n\n提示: 进化需要 task_description。当前未进化决策均无 task_description。");
                    } else if total == 0 {
                        let all_count: i64 = conn
                            .query_row("SELECT COUNT(*) FROM decisions", [], |r| r.get(0))
                            .unwrap_or(0);
                        if all_count > 0 {
                            hint.push_str(
                                "\n\n提示: 未进化决策队列为空。已有决策均已标记为已进化。",
                            );
                            hint.push_str("\n请执行新任务积累新决策后再触发进化。");
                        } else {
                            hint.push_str("\n\n提示: 进化队列为空。请先运行 skilllite run 或 skilllite chat 积累决策。");
                        }
                    }
                }
            }
            hint
        }
        Err(e) => return Err(format!("执行 evolution run 失败: {}", e)),
    };

    let mut clipped = response.clone();
    if clipped.len() > 480 {
        clipped.truncate(480);
        clipped.push('…');
    }
    if let Ok(conn) =
        skilllite_evolution::feedback::open_evolution_db(&skilllite_core::paths::chat_root())
    {
        let _ = skilllite_evolution::log_evolution_event(
            &conn,
            &skilllite_core::paths::chat_root(),
            "manual_evolution_run_triggered",
            proposal_id.unwrap_or("all"),
            &clipped,
            workspace,
        );
    }
    Ok(clipped)
}
