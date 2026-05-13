//! Inbound HTTP surface for messaging webhooks (MVP), without a separate `skilllite-gateway` crate.
//!
//! Binds only when **`SKILLLITE_CHANNEL_SERVE_ALLOW=1`** (same safety pattern as `artifact-serve`).
//! Optional outbound summaries: configure any of the `SKILLLITE_CHANNEL_*` env vars below; each
//! accepted `POST /webhook/inbound` triggers **best-effort** notifications (failures are logged only).
//!
//! Supported optional bindings:
//! - DingTalk: `SKILLLITE_CHANNEL_DINGTALK_WEBHOOK` (+ optional `SKILLLITE_CHANNEL_DINGTALK_SECRET`)
//! - Feishu / Lark: `SKILLLITE_CHANNEL_FEISHU_WEBHOOK` (+ optional `SKILLLITE_CHANNEL_FEISHU_SECRET`)
//! - Telegram: `SKILLLITE_CHANNEL_TELEGRAM_BOT_TOKEN` + `SKILLLITE_CHANNEL_TELEGRAM_CHAT_ID`

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header::AUTHORIZATION, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde_json::json;
use tower_http::trace::TraceLayer;

use crate::error::{bail, Error, Result};
use skilllite_core::config::env_keys::channel as channel_keys;

/// Env var that must equal `1` before `channel serve` will bind (CLI only).
pub const CHANNEL_SERVE_ALLOW_ENV: &str = channel_keys::SKILLLITE_CHANNEL_SERVE_ALLOW;

/// Upper bound for outbound preview text (shared across IM platforms).
const INBOUND_NOTIFY_MAX_CHARS: usize = 3500;

#[derive(Clone)]
struct AppState {
    bearer: Option<String>,
}

/// Run the HTTP server until process is interrupted (Ctrl+C).
pub async fn run_channel_http_server(addr: SocketAddr, bearer: Option<String>) -> Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .merge(channel_webhook_router(bearer))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| Error::validation(format!("bind {addr}: {e}")))?;

    let local = listener
        .local_addr()
        .map_err(|e| Error::validation(e.to_string()))?;
    eprintln!("{}={local}", channel_keys::SKILLLITE_CHANNEL_HTTP_ADDR);
    eprintln!(
        "channel serve: GET /health  POST /webhook/inbound  (set {}=1 to allow bind)",
        CHANNEL_SERVE_ALLOW_ENV
    );

    axum::serve(listener, app)
        .await
        .map_err(|e| Error::validation(format!("server: {e}")))
}

/// Sync entry for CLI: builds a multi-thread runtime and blocks until shutdown.
pub fn cmd_channel_serve(bind: &str, token: Option<&str>) -> Result<()> {
    let allowed = std::env::var(CHANNEL_SERVE_ALLOW_ENV)
        .map(|v| v.trim() == "1")
        .unwrap_or(false);
    if !allowed {
        bail!(
            "refusing to start channel HTTP server: set {}=1 to bind (avoids accidental network exposure)",
            CHANNEL_SERVE_ALLOW_ENV
        );
    }

    let addr: SocketAddr = bind
        .parse()
        .map_err(|e| Error::validation(format!("invalid --bind {bind:?}: {e}")))?;

    if !addr.ip().is_loopback() && token.map(str::is_empty).unwrap_or(true) {
        let insecure = std::env::var(channel_keys::SKILLLITE_CHANNEL_HTTP_ALLOW_INSECURE_NO_AUTH)
            .map(|v| v.trim() == "1")
            .unwrap_or(false);
        if !insecure {
            bail!(
                "non-loopback --bind requires --token or set {}=1 (unsafe)",
                channel_keys::SKILLLITE_CHANNEL_HTTP_ALLOW_INSECURE_NO_AUTH
            );
        }
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| Error::validation(format!("tokio runtime: {e}")))?;
    rt.block_on(run_channel_http_server(addr, token.map(|s| s.to_string())))
}

/// Reusable inbound webhook router for embedding into a larger HTTP host.
pub fn channel_webhook_router(bearer: Option<String>) -> Router {
    let state = AppState { bearer };
    Router::new()
        .route("/webhook/inbound", post(webhook_inbound))
        .with_state(Arc::new(state))
}

async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(json!({ "ok": true, "service": "skilllite-channel-serve" })),
    )
}

async fn webhook_inbound(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    if let Some(expected) = &state.bearer {
        let ok = headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.strip_prefix("Bearer ").unwrap_or(v).trim() == expected.as_str())
            .unwrap_or(false);
        if !ok {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(json!({ "error": "unauthorized" })),
            )
                .into_response();
        }
    }

    let preview = safe_preview_body(&body);
    tracing::info!(len = body.len(), "webhook inbound");

    maybe_notify_inbound_preview(&preview).await;

    (
        StatusCode::OK,
        axum::Json(json!({
            "ok": true,
            "received_bytes": body.len(),
            "preview": preview,
        })),
    )
        .into_response()
}

fn safe_preview_body(body: &Bytes) -> String {
    let s = String::from_utf8_lossy(body);
    let n = 400usize;
    if s.chars().count() <= n {
        s.into_owned()
    } else {
        s.chars().take(n).collect::<String>() + "…"
    }
}

fn clamp_notify_body(s: &str) -> String {
    if s.chars().count() <= INBOUND_NOTIFY_MAX_CHARS {
        return s.to_string();
    }
    s.chars().take(INBOUND_NOTIFY_MAX_CHARS).collect::<String>() + "…"
}

/// Best-effort outbound summaries (DingTalk / Feishu / Telegram); each reads its own env vars.
async fn maybe_notify_inbound_preview(preview: &str) {
    let body = clamp_notify_body(preview);
    maybe_notify_dingtalk(&body).await;
    maybe_notify_feishu(&body).await;
    maybe_notify_telegram(&body).await;
}

async fn spawn_blocking_notify(
    platform: &'static str,
    job: impl FnOnce() -> skilllite_channel::Result<()> + Send + 'static,
) {
    match tokio::task::spawn_blocking(job).await {
        Ok(Ok(())) => {}
        Ok(Err(e)) => tracing::warn!(%platform, error = %e, "optional channel notify failed"),
        Err(e) => tracing::warn!(%platform, error = %e, "channel notify join failed"),
    }
}

async fn maybe_notify_dingtalk(preview: &str) {
    let url = match std::env::var(channel_keys::SKILLLITE_CHANNEL_DINGTALK_WEBHOOK) {
        Ok(s) if !s.trim().is_empty() => s,
        _ => return,
    };
    let secret = std::env::var(channel_keys::SKILLLITE_CHANNEL_DINGTALK_SECRET).ok();
    let preview = preview.to_string();
    spawn_blocking_notify("dingtalk", move || {
        let robot = skilllite_channel::DingTalkRobot::new(url, secret)?;
        let text = format!("Inbound webhook:\n```\n{preview}\n```");
        robot.send_markdown("SkillLite channel serve", &text)
    })
    .await;
}

async fn maybe_notify_feishu(preview: &str) {
    let url = match std::env::var(channel_keys::SKILLLITE_CHANNEL_FEISHU_WEBHOOK) {
        Ok(s) if !s.trim().is_empty() => s,
        _ => return,
    };
    let secret = std::env::var(channel_keys::SKILLLITE_CHANNEL_FEISHU_SECRET).ok();
    let preview = preview.to_string();
    spawn_blocking_notify("feishu", move || {
        let bot = skilllite_channel::FeishuBot::new(url, secret)?;
        let text = format!("Inbound webhook (SkillLite)\n```\n{preview}\n```");
        bot.send_text(&text)
    })
    .await;
}

async fn maybe_notify_telegram(preview: &str) {
    let token = match std::env::var(channel_keys::SKILLLITE_CHANNEL_TELEGRAM_BOT_TOKEN) {
        Ok(s) if !s.trim().is_empty() => s,
        _ => return,
    };
    let chat_id = match std::env::var(channel_keys::SKILLLITE_CHANNEL_TELEGRAM_CHAT_ID) {
        Ok(s) if !s.trim().is_empty() => s,
        _ => return,
    };
    let preview = preview.to_string();
    spawn_blocking_notify("telegram", move || {
        let bot = skilllite_channel::TelegramBot::new(token, chat_id)?;
        let text = format!("Inbound webhook (SkillLite)\n```\n{preview}\n```");
        bot.send_text(&text)
    })
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_truncates_unicode_without_panic() {
        let s = "你好".repeat(300);
        let b = Bytes::from(s);
        let p = safe_preview_body(&b);
        assert!(p.contains('…') || p.len() <= 800);
        assert!(p.contains("你好"));
    }

    #[test]
    fn clamp_notify_respects_max_chars() {
        let s = "a".repeat(INBOUND_NOTIFY_MAX_CHARS + 50);
        let out = clamp_notify_body(&s);
        assert!(out.ends_with('…'));
        assert!(out.chars().count() <= INBOUND_NOTIFY_MAX_CHARS + 1);
    }

    #[test]
    fn cmd_serve_refuses_without_allow_env() {
        std::env::remove_var(CHANNEL_SERVE_ALLOW_ENV);
        let r = cmd_channel_serve("127.0.0.1:0", None);
        assert!(r.is_err());
        let msg = r.expect_err("err").to_string();
        assert!(msg.contains(CHANNEL_SERVE_ALLOW_ENV), "unexpected: {msg}");
    }
}
