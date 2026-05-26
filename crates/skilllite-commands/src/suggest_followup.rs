//! `skilllite suggest-followup --json` — desktop follow-up chip suggestions (L2).

use std::collections::HashMap;

use serde::Serialize;
use skilllite_agent::llm::LlmClient;
use skilllite_agent::types::ChatMessage;
use skilllite_core::config::env_keys::llm as llm_keys;
use skilllite_core::config::{parse_dotenv_from_dir, LlmConfig};
use skilllite_evolution::sanitize_visible_llm_text;

use crate::evolution_status::resolve_workspace_root;
use crate::Result;

#[derive(Debug, Serialize)]
pub struct SuggestFollowupResponse {
    pub suggestions: Vec<String>,
}

fn env_lookup(map: &HashMap<String, String>, primary: &str, aliases: &[&str]) -> Option<String> {
    for k in std::iter::once(primary).chain(aliases.iter().copied()) {
        if let Some(v) = map.get(k) {
            let t = v.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

fn llm_from_env(workspace_root: &std::path::Path) -> Result<(String, String, String)> {
    let mut map: HashMap<String, String> = parse_dotenv_from_dir(workspace_root)
        .into_iter()
        .collect();
    for (k, v) in std::env::vars() {
        map.entry(k).or_insert(v);
    }
    let api_key = env_lookup(&map, llm_keys::API_KEY, llm_keys::API_KEY_ALIASES)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| crate::Error::validation("API key not configured".to_string()))?;
    let api_base = env_lookup(&map, llm_keys::API_BASE, llm_keys::API_BASE_ALIASES)
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
    let model = env_lookup(&map, llm_keys::MODEL, llm_keys::MODEL_ALIASES)
        .unwrap_or_else(|| LlmConfig::default_model_for_base(&api_base).to_string());
    Ok((api_base, api_key, model))
}

fn last_user_message_text(transcript: &str) -> Option<&str> {
    let mut last: Option<&str> = None;
    for block in transcript.split("\n\n") {
        let b = block.trim();
        if let Some(rest) = b.strip_prefix("User:") {
            let t = rest.trim();
            if !t.is_empty() {
                last = Some(t);
            }
        }
    }
    last
}

fn contains_non_latin_script(s: &str) -> bool {
    s.chars().any(|c| {
        let u = u32::from(c);
        (0x4E00..=0x9FFF).contains(&u)
            || (0x3040..=0x30FF).contains(&u)
            || (0xAC00..=0xD7AF).contains(&u)
    })
}

fn followup_language_clause(last_user: Option<&str>) -> &'static str {
    let Some(s) = last_user else {
        return "";
    };
    if contains_non_latin_script(s) {
        "\n\n【Language — mandatory】The user's last message uses Chinese/Japanese/Korean (CJK). Write all 3 follow-up questions in the same language as that message (use Simplified Chinese if the user wrote Chinese). Do not answer in English.\n【语言 — 必须遵守】最后一条用户消息含中日韩文字。三条后续提问必须与该条用户消息使用同一种自然语言（用户用中文则用简体中文）。禁止仅用英文输出。"
    } else {
        let ascii_letters = s.chars().filter(|c| c.is_ascii_alphabetic()).count();
        if ascii_letters >= 3 {
            "\n\n【Language — mandatory】The user's last message is in English (Latin script). Write all 3 follow-up questions in English.\n【语言 — 必须遵守】最后一条用户消息为英文。三条后续提问必须使用英文。"
        } else {
            "\n\n【Language — mandatory】Match the natural language of the user's last message in the transcript above (not the assistant's). If it is Chinese, use Chinese; if English, use English."
        }
    }
}

fn is_meta_followup_line(t: &str) -> bool {
    let lower = t.to_ascii_lowercase();
    let ascii_meta = [
        "the user's last message",
        "user's last message",
        "i need to generate",
        "generate exactly 3",
        "write all 3 follow-up",
        "follow-up questions in",
        "mandatory language",
    ];
    if ascii_meta.iter().any(|n| lower.contains(n)) {
        return true;
    }
    let cjk_meta = [
        "用户最后的消息",
        "所以我需要",
        "最后一条用户消息",
        "三条后续提问",
        "必须与该条用户消息",
        "禁止仅用英文",
        "用户已经:",
        "这是在要求",
    ];
    cjk_meta.iter().any(|n| t.contains(n))
}

fn parse_suggestion_lines(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in raw.lines() {
        let mut t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(rest) = t.strip_prefix("```") {
            t = rest.trim();
        }
        t = t.trim_end_matches('`').trim();
        let t = t
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .trim_start_matches(['.', ')', '、'])
            .trim();
        let t = t
            .trim_start_matches("- ")
            .trim_start_matches("• ")
            .trim_start_matches("* ")
            .trim();
        if !t.is_empty() && !is_meta_followup_line(t) {
            out.push(t.to_string());
        }
        if out.len() >= 3 {
            break;
        }
    }
    out
}

pub async fn suggest_followup(
    workspace: &str,
    transcript: &str,
) -> Result<SuggestFollowupResponse> {
    let trimmed = transcript.trim();
    if trimmed.is_empty() {
        return Ok(SuggestFollowupResponse {
            suggestions: vec![],
        });
    }
    let root = resolve_workspace_root(workspace);
    for (k, v) in parse_dotenv_from_dir(&root) {
        if std::env::var(&k).is_err() {
            std::env::set_var(&k, &v);
        }
    }
    let (api_base, api_key, model) = llm_from_env(&root)?;
    let client = LlmClient::new(&api_base, &api_key).map_err(crate::Error::Agent)?;

    let body: String = trimmed.chars().take(12_000).collect();
    let last_user = last_user_message_text(trimmed);
    let lang = followup_language_clause(last_user);
    let user_msg = format!(
        "Here is a condensed transcript of a chat session that just ended (User / Assistant turns only):\n\n\"\"\"\n{body}\n\"\"\"\n\nGenerate exactly 3 short follow-up questions the user might want to ask next.\nRules:\n- The assistant's replies may be in English; ignore that for language choice. Use only the **user's** messages to decide the output language (especially the **last user message**).\n- Each question must suggest a new direction or refinement; do not repeat what was already settled.\n- Output exactly 3 lines: one question per line. No numbering, no bullets, no preamble or closing.{lang}"
    );

    let messages = vec![
        ChatMessage::system(
            "You suggest short follow-up questions after an agent chat. The transcript mixes languages; you MUST follow the mandatory Language section in the user message (based on the last user turn), not the assistant's language.",
        ),
        ChatMessage::user(&user_msg),
    ];

    let resp = client
        .chat_completion(&model, &messages, None, Some(0.4), None)
        .await
        .map_err(crate::Error::Agent)?;

    let raw = resp
        .choices
        .first()
        .and_then(|c| c.message.content.as_deref())
        .unwrap_or("")
        .trim();
    let text = sanitize_visible_llm_text(raw);
    Ok(SuggestFollowupResponse {
        suggestions: parse_suggestion_lines(&text),
    })
}

pub fn cmd_suggest_followup(json: bool, workspace: &str, transcript: Option<&str>) -> Result<()> {
    let body = match transcript {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => {
            let mut s = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut s)?;
            s
        }
    };
    let rt = tokio::runtime::Runtime::new().map_err(|e| crate::Error::Validation(e.to_string()))?;
    let resp = rt.block_on(suggest_followup(workspace, &body))?;
    if json {
        println!("{}", serde_json::to_string(&resp)?);
    } else {
        for line in &resp.suggestions {
            println!("{line}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_suggestion_lines;

    #[test]
    fn parses_numbered_and_bullets() {
        let raw = "1. First question?\n2. Second?\n- Third one?\n";
        let v = parse_suggestion_lines(raw);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], "First question?");
    }
}
