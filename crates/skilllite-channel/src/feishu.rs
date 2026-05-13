//! Feishu (Lark) custom bot webhook — text messages with optional signature.
//!
//! Signing algorithm (custom bot “签名校验”): `string_to_sign = "{timestamp}\n{secret}"` is used as
//! the **HMAC-SHA256 key**; the HMAC **message is empty**. `timestamp` is Unix time in **seconds**
//! (string in JSON). See Feishu docs: <https://open.feishu.cn/document/client-docs/bot-v3/add-custom-bot>

#[cfg(feature = "http")]
use base64::{engine::general_purpose::STANDARD, Engine};
#[cfg(feature = "http")]
use hmac::{Hmac, Mac};
#[cfg(feature = "http")]
use serde::Serialize;
#[cfg(feature = "http")]
use sha2::Sha256;

#[cfg(feature = "http")]
use crate::error::{bail, Error, Result};
#[cfg(feature = "http")]
use crate::http_common::http_client;

type HmacSha256 = Hmac<Sha256>;

/// Feishu / Lark custom robot client (bot v2 hook URL).
#[cfg(feature = "http")]
#[derive(Debug, Clone)]
pub struct FeishuBot {
    webhook_url: String,
    secret: Option<String>,
}

#[cfg(feature = "http")]
impl FeishuBot {
    /// `webhook_url` example: `https://open.feishu.cn/open-apis/bot/v2/hook/xxxxxxxx`
    pub fn new(webhook_url: impl Into<String>, secret: Option<String>) -> Result<Self> {
        let webhook_url = webhook_url.into();
        let webhook_url = webhook_url.trim().to_string();
        if webhook_url.is_empty() {
            bail!("Feishu webhook URL must not be empty");
        }
        if !webhook_url.to_ascii_lowercase().starts_with("https://") {
            return Err(Error::validation("Feishu webhook URL must use HTTPS"));
        }
        Ok(Self {
            webhook_url,
            secret,
        })
    }

    /// Sends a `text` message (`msg_type`: `text`).
    pub fn send_text(&self, text: &str) -> Result<()> {
        let (timestamp, sign) = if let Some(secret) = &self.secret {
            let secret = secret.trim();
            if secret.is_empty() {
                bail!("Feishu secret is set but empty");
            }
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| Error::validation(format!("system clock: {e}")))?
                .as_secs();
            let sign = feishu_sign(secret, ts)?;
            (Some(ts.to_string()), Some(sign))
        } else {
            (None, None)
        };

        let body = FeishuTextRequest {
            timestamp,
            sign,
            msg_type: "text",
            content: FeishuTextContent {
                text: text.to_string(),
            },
        };
        let client = http_client()?;
        let resp = client
            .post(&self.webhook_url)
            .json(&body)
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .map_err(|e| Error::http(e.to_string()))?;
        map_feishu_response(resp)
    }
}

#[cfg(feature = "http")]
#[derive(Serialize)]
struct FeishuTextRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign: Option<String>,
    msg_type: &'static str,
    content: FeishuTextContent,
}

#[cfg(feature = "http")]
#[derive(Serialize)]
struct FeishuTextContent {
    text: String,
}

#[cfg(feature = "http")]
fn map_feishu_response(resp: reqwest::blocking::Response) -> Result<()> {
    let status = resp.status();
    let bytes = resp.bytes().map_err(|e| Error::http(e.to_string()))?;
    if !status.is_success() {
        return Err(Error::http(format!(
            "Feishu HTTP {}: {}",
            status.as_u16(),
            String::from_utf8_lossy(&bytes)
        )));
    }
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    if let Some(c) = v.get("code").and_then(|x| x.as_i64()) {
        if c == 0 {
            return Ok(());
        }
        let msg = v
            .get("msg")
            .and_then(|x| x.as_str())
            .unwrap_or("unknown error");
        return Err(Error::http(format!("Feishu code={c}: {msg}")));
    }
    if let Some(sc) = v.get("StatusCode").and_then(|x| x.as_i64()) {
        if sc == 0 {
            return Ok(());
        }
    }
    Ok(())
}

/// Feishu custom-bot signature (`timestamp` in **seconds**).
#[cfg(feature = "http")]
pub fn feishu_sign(secret: &str, timestamp_sec: u64) -> Result<String> {
    if secret.is_empty() {
        return Err(Error::validation("Feishu secret must not be empty"));
    }
    let string_to_sign = format!("{timestamp_sec}\n{secret}");
    let mut mac = HmacSha256::new_from_slice(string_to_sign.as_bytes())
        .map_err(|e| Error::validation(format!("invalid Feishu signing key material: {e}")))?;
    mac.update(&[]);
    Ok(STANDARD.encode(mac.finalize().into_bytes()))
}
