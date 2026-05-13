//! Telegram Bot API — `sendMessage` for plain text notifications.
//!
//! Docs: <https://core.telegram.org/bots/api#sendmessage>

#[cfg(feature = "http")]
use serde::Serialize;

#[cfg(feature = "http")]
use crate::error::{bail, Error, Result};
#[cfg(feature = "http")]
use crate::http_common::http_client;

/// Minimal Telegram bot client (`sendMessage`).
#[cfg(feature = "http")]
#[derive(Debug, Clone)]
pub struct TelegramBot {
    token: String,
    chat_id: String,
}

#[cfg(feature = "http")]
impl TelegramBot {
    /// `token` is the bot token; `chat_id` is a numeric id, `@channelusername`, or `-100…` supergroup id string.
    pub fn new(token: impl Into<String>, chat_id: impl Into<String>) -> Result<Self> {
        let token = token.into();
        let token = token.trim().to_string();
        if token.is_empty() {
            bail!("Telegram bot token must not be empty");
        }
        if token.contains(char::is_whitespace) {
            return Err(Error::validation(
                "Telegram bot token must not contain whitespace",
            ));
        }
        let chat_id = chat_id.into();
        let chat_id = chat_id.trim().to_string();
        if chat_id.is_empty() {
            bail!("Telegram chat_id must not be empty");
        }
        Ok(Self { token, chat_id })
    }

    fn send_message_url(&self) -> String {
        format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.token
        )
    }

    /// Sends a plain-text message (no `parse_mode`; avoids Markdown entity escaping issues).
    pub fn send_text(&self, text: &str) -> Result<()> {
        let body = TelegramSendMessage {
            chat_id: chat_id_to_json(&self.chat_id)?,
            text: text.to_string(),
            disable_web_page_preview: true,
        };
        let client = http_client()?;
        let resp = client
            .post(self.send_message_url())
            .json(&body)
            .send()
            .map_err(|e| Error::http(e.to_string()))?;
        map_telegram_response(resp)
    }
}

#[cfg(feature = "http")]
#[derive(Serialize)]
struct TelegramSendMessage {
    chat_id: serde_json::Value,
    text: String,
    disable_web_page_preview: bool,
}

#[cfg(feature = "http")]
fn chat_id_to_json(s: &str) -> Result<serde_json::Value> {
    let t = s.trim();
    if let Ok(n) = t.parse::<i64>() {
        return Ok(serde_json::json!(n));
    }
    Ok(serde_json::Value::String(t.to_string()))
}

#[cfg(feature = "http")]
fn map_telegram_response(resp: reqwest::blocking::Response) -> Result<()> {
    let status = resp.status();
    let bytes = resp.bytes().map_err(|e| Error::http(e.to_string()))?;
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    if !status.is_success() {
        let desc = v
            .get("description")
            .and_then(|x| x.as_str())
            .unwrap_or("unknown error");
        return Err(Error::http(format!(
            "Telegram HTTP {}: {desc}",
            status.as_u16()
        )));
    }
    let ok = v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false);
    if ok {
        return Ok(());
    }
    let desc = v
        .get("description")
        .and_then(|x| x.as_str())
        .unwrap_or("unknown error");
    Err(Error::http(format!("Telegram API: {desc}")))
}
