//! 环境变量 key 常量与别名定义
//!
//! 主变量优先使用 `SKILLLITE_*`，兼容 `OPENAI_*`、`SKILLBOX_*` 等。

// ─── Legacy flat constants (backward compat: env/builder, etc.) ──────────────
pub const SKILLLITE_CACHE_DIR: &str = "SKILLLITE_CACHE_DIR";
pub const SKILLLITE_OUTPUT_DIR: &str = "SKILLLITE_OUTPUT_DIR";
pub const SKILLLITE_SKILLS_DIR: &str = "SKILLLITE_SKILLS_DIR";
pub const SKILLLITE_MODEL: &str = "SKILLLITE_MODEL";
pub const SKILLLITE_QUIET: &str = "SKILLLITE_QUIET";
pub const SKILLLITE_LOG_LEVEL: &str = "SKILLLITE_LOG_LEVEL";
pub const SKILLLITE_ENABLE_TASK_PLANNING: &str = "SKILLLITE_ENABLE_TASK_PLANNING";
pub const SKILLBOX_SKILLS_ROOT: &str = "SKILLBOX_SKILLS_ROOT";
pub const SKILLBOX_CACHE_DIR: &str = "SKILLBOX_CACHE_DIR";
pub const AGENTSKILL_CACHE_DIR: &str = "AGENTSKILL_CACHE_DIR";

/// LLM API 配置
pub mod llm {
    /// API Base — 主变量优先
    pub const API_BASE: &str = "SKILLLITE_API_BASE";
    pub const API_BASE_ALIASES: &[&str] = &["OPENAI_API_BASE", "OPENAI_BASE_URL", "BASE_URL"];

    /// API Key
    pub const API_KEY: &str = "SKILLLITE_API_KEY";
    pub const API_KEY_ALIASES: &[&str] = &["OPENAI_API_KEY", "API_KEY"];

    /// Model
    pub const MODEL: &str = "SKILLLITE_MODEL";
    pub const MODEL_ALIASES: &[&str] = &["OPENAI_MODEL", "MODEL"];
}

/// Skills、输出、工作区
pub mod paths {
    pub const SKILLLITE_SKILLS_DIR: &str = "SKILLLITE_SKILLS_DIR";
    pub const SKILLS_DIR_ALIASES: &[&str] = &["SKILLS_DIR"];

    pub const SKILLLITE_OUTPUT_DIR: &str = "SKILLLITE_OUTPUT_DIR";

    pub const SKILLLITE_WORKSPACE: &str = "SKILLLITE_WORKSPACE";

    pub const SKILLLITE_SKILLS_REPO: &str = "SKILLLITE_SKILLS_REPO";

    pub const SKILLBOX_SKILLS_ROOT: &str = "SKILLBOX_SKILLS_ROOT";

    /// Sandbox child-process marker. Set to `1` by sandbox launcher; consulted
    /// by inner code paths to detect "running under sandbox".
    pub const SKILLLITE_SANDBOX: &str = "SKILLLITE_SANDBOX";

    /// Sandbox child-process flag: when sandbox launcher disables the
    /// network for the child, this is set to `1` so inner code can adapt.
    pub const SKILLLITE_NETWORK_DISABLED: &str = "SKILLLITE_NETWORK_DISABLED";
}

/// `skilllite schedule tick`：非 dry-run 时是否允许调用 LLM（默认视为关闭，需显式开启）
pub mod schedule {
    pub const SKILLLITE_SCHEDULE_ENABLED: &str = "SKILLLITE_SCHEDULE_ENABLED";
}

/// 缓存目录
pub mod cache {
    pub const SKILLLITE_CACHE_DIR: &str = "SKILLLITE_CACHE_DIR";
    pub const CACHE_DIR_ALIASES: &[&str] = &["SKILLBOX_CACHE_DIR", "AGENTSKILL_CACHE_DIR"];
}

/// 可观测性与日志
pub mod observability {
    pub const SKILLLITE_QUIET: &str = "SKILLLITE_QUIET";
    pub const QUIET_ALIASES: &[&str] = &["SKILLBOX_QUIET"];

    pub const SKILLLITE_LOG_LEVEL: &str = "SKILLLITE_LOG_LEVEL";
    pub const LOG_LEVEL_ALIASES: &[&str] = &["SKILLBOX_LOG_LEVEL"];

    pub const SKILLLITE_LOG_JSON: &str = "SKILLLITE_LOG_JSON";
    pub const LOG_JSON_ALIASES: &[&str] = &["SKILLBOX_LOG_JSON"];

    pub const SKILLLITE_AUDIT_LOG: &str = "SKILLLITE_AUDIT_LOG";
    pub const AUDIT_LOG_ALIASES: &[&str] = &["SKILLBOX_AUDIT_LOG"];

    /// 设为 1/true 时显式关闭审计（默认开启）
    pub const SKILLLITE_AUDIT_DISABLED: &str = "SKILLLITE_AUDIT_DISABLED";

    pub const SKILLLITE_SECURITY_EVENTS_LOG: &str = "SKILLLITE_SECURITY_EVENTS_LOG";

    /// P0 可观测 vs P1 可阻断：设为 1/true 时，HashChanged/SignatureInvalid/TrustDeny 会阻断执行；不设或 0 时仅展示状态不阻断（P0 模式）
    pub const SKILLLITE_SUPPLY_CHAIN_BLOCK: &str = "SKILLLITE_SUPPLY_CHAIN_BLOCK";

    /// 审计上下文：谁调用了 Skill（如 session_id、invoker），用于 audit 日志
    pub const SKILLLITE_AUDIT_CONTEXT: &str = "SKILLLITE_AUDIT_CONTEXT";

    /// 逗号分隔的 SKILL 名称列表，与 `skill-denylist.txt` 合并；命中则执行前拒绝（P1 手动禁用）
    pub const SKILLLITE_SKILL_DENYLIST: &str = "SKILLLITE_SKILL_DENYLIST";

    /// `skilllite audit-report --alert` 或需 webhook 时的告警 URL
    pub const SKILLLITE_AUDIT_ALERT_WEBHOOK: &str = "SKILLLITE_AUDIT_ALERT_WEBHOOK";

    /// 时间窗内单 Skill 调用次数超过此值则预警（默认 200）
    pub const SKILLLITE_AUDIT_ALERT_MAX_INVOCATIONS_PER_SKILL: &str =
        "SKILLLITE_AUDIT_ALERT_MAX_INVOCATIONS_PER_SKILL";

    /// 参与「失败率」判定的最少调用次数（默认 5）
    pub const SKILLLITE_AUDIT_ALERT_MIN_INVOCATIONS_FOR_FAILURE: &str =
        "SKILLLITE_AUDIT_ALERT_MIN_INVOCATIONS_FOR_FAILURE";

    /// 失败率不低于此值则预警，0.0–1.0（默认 0.5）
    pub const SKILLLITE_AUDIT_ALERT_FAILURE_RATIO: &str = "SKILLLITE_AUDIT_ALERT_FAILURE_RATIO";

    /// 时间窗内 edit 事件触及的不重复路径数超过此值则预警（默认 80）
    pub const SKILLLITE_AUDIT_ALERT_EDIT_UNIQUE_PATHS: &str =
        "SKILLLITE_AUDIT_ALERT_EDIT_UNIQUE_PATHS";
}

/// Agent 行为提示（桌面 / CLI 可选）
pub mod agent {
    /// 桌面或包装器传入的界面语言：`zh` | `en`。合并进聊天类 system prompt 的附加段（与 `AgentConfig.context_append` 同源逻辑）。
    pub const SKILLLITE_UI_LOCALE: &str = "SKILLLITE_UI_LOCALE";
}

/// Memory 向量检索
pub mod memory {
    pub const SKILLLITE_EMBEDDING_MODEL: &str = "SKILLLITE_EMBEDDING_MODEL";
    pub const SKILLLITE_EMBEDDING_DIMENSION: &str = "SKILLLITE_EMBEDDING_DIMENSION";
    /// Optional separate embedding API base; falls back to LLM `API_BASE` chain.
    pub const SKILLLITE_EMBEDDING_BASE_URL: &str = "SKILLLITE_EMBEDDING_BASE_URL";
    /// Optional separate embedding API key; falls back to LLM `API_KEY` chain.
    pub const SKILLLITE_EMBEDDING_API_KEY: &str = "SKILLLITE_EMBEDDING_API_KEY";

    /// Master switch: enable conversation memory subsystem (default `true`).
    pub const SKILLLITE_ENABLE_MEMORY: &str = "SKILLLITE_ENABLE_MEMORY";
    /// Switch: enable the vector-search backend for memory (default `false`).
    pub const SKILLLITE_ENABLE_MEMORY_VECTOR: &str = "SKILLLITE_ENABLE_MEMORY_VECTOR";
}

/// Long-text summarization & context-window tuning (used by `agent::types::env_config`)
pub mod summarization {
    /// Chunk size (chars) for splitting long inputs.
    pub const SKILLLITE_CHUNK_SIZE: &str = "SKILLLITE_CHUNK_SIZE";
    /// Number of head chunks for head+tail strategy.
    pub const SKILLLITE_HEAD_CHUNKS: &str = "SKILLLITE_HEAD_CHUNKS";
    /// Number of tail chunks for head+tail strategy.
    pub const SKILLLITE_TAIL_CHUNKS: &str = "SKILLLITE_TAIL_CHUNKS";
    /// Max output chars for chunked summarization.
    pub const SKILLLITE_MAX_OUTPUT_CHARS: &str = "SKILLLITE_MAX_OUTPUT_CHARS";
    /// Cheaper model for the Map stage of MapReduce summarization.
    pub const SKILLLITE_MAP_MODEL: &str = "SKILLLITE_MAP_MODEL";
    /// Long-text strategy: `head_tail_only` | `head_tail_extract` | `mapreduce_full`.
    pub const SKILLLITE_LONG_TEXT_STRATEGY: &str = "SKILLLITE_LONG_TEXT_STRATEGY";
    /// Ratio of total chunks selected in extract mode.
    pub const SKILLLITE_EXTRACT_TOP_K_RATIO: &str = "SKILLLITE_EXTRACT_TOP_K_RATIO";
    /// Threshold (chars) above which chunked summarization is preferred.
    pub const SKILLLITE_SUMMARIZE_THRESHOLD: &str = "SKILLLITE_SUMMARIZE_THRESHOLD";
    /// Max LLM completion tokens.
    pub const SKILLLITE_MAX_TOKENS: &str = "SKILLLITE_MAX_TOKENS";
    /// Max chars for a single user input message before truncation/summarization.
    pub const SKILLLITE_USER_INPUT_MAX_CHARS: &str = "SKILLLITE_USER_INPUT_MAX_CHARS";
    /// Max chars per generic tool result.
    pub const SKILLLITE_TOOL_RESULT_MAX_CHARS: &str = "SKILLLITE_TOOL_RESULT_MAX_CHARS";
    /// Max chars for a single `read_file` tool result.
    pub const SKILLLITE_READ_FILE_TOOL_RESULT_MAX_CHARS: &str =
        "SKILLLITE_READ_FILE_TOOL_RESULT_MAX_CHARS";
    /// Max chars for tool messages during context-overflow recovery.
    pub const SKILLLITE_TOOL_RESULT_RECOVERY_MAX_CHARS: &str =
        "SKILLLITE_TOOL_RESULT_RECOVERY_MAX_CHARS";
    /// Soft limit on conversation payload before main agent LLM call.
    pub const SKILLLITE_CONTEXT_SOFT_LIMIT_CHARS: &str = "SKILLLITE_CONTEXT_SOFT_LIMIT_CHARS";
    /// Compaction threshold (message count).
    pub const SKILLLITE_COMPACTION_THRESHOLD: &str = "SKILLLITE_COMPACTION_THRESHOLD";
    /// Pre-compaction memory flush enabled flag.
    pub const SKILLLITE_MEMORY_FLUSH_ENABLED: &str = "SKILLLITE_MEMORY_FLUSH_ENABLED";
    /// Memory flush threshold (message count).
    pub const SKILLLITE_MEMORY_FLUSH_THRESHOLD: &str = "SKILLLITE_MEMORY_FLUSH_THRESHOLD";
    /// Number of recent messages to keep after compaction.
    pub const SKILLLITE_COMPACTION_KEEP_RECENT: &str = "SKILLLITE_COMPACTION_KEEP_RECENT";
    /// Window size (messages) for in-flight history truncation.
    pub const SKILLLITE_HISTORY_WINDOW_MESSAGES: &str = "SKILLLITE_HISTORY_WINDOW_MESSAGES";
}

/// MCP client / server bootstrap
pub mod mcp {
    /// Inline JSON document carrying the desktop-side MCP server configuration.
    pub const SKILLLITE_MCP_SERVERS_JSON: &str = "SKILLLITE_MCP_SERVERS_JSON";
    /// Set to `0` to disable the agent's built-in MCP client bootstrap.
    pub const SKILLLITE_AGENT_MCP_CLIENT: &str = "SKILLLITE_AGENT_MCP_CLIENT";
}

/// Goal extraction / agent loop heuristics
pub mod goal {
    /// Set to `1` to enable LLM fallback when regex-based goal extraction is empty.
    pub const SKILLLITE_GOAL_LLM_EXTRACT: &str = "SKILLLITE_GOAL_LLM_EXTRACT";
}

/// 进化引擎
pub mod evolution {
    /// Evolution mode: "1" (default, all), "prompts", "memory", "skills", "0" (disabled).
    pub const SKILLLITE_EVOLUTION: &str = "SKILLLITE_EVOLUTION";
    pub const SKILLLITE_MAX_EVOLUTIONS_PER_DAY: &str = "SKILLLITE_MAX_EVOLUTIONS_PER_DAY";
    /// A9: Periodic evolution interval (seconds). Default 600 (10 min). Used by `ChatSession` and desktop Life Pulse.
    pub const SKILLLITE_EVOLUTION_INTERVAL_SECS: &str = "SKILLLITE_EVOLUTION_INTERVAL_SECS";
    /// A9: OR-trigger — raw unprocessed decision rows (`evolved = 0`, default 10). Used with weighted signal arm.
    pub const SKILLLITE_EVOLUTION_DECISION_THRESHOLD: &str =
        "SKILLLITE_EVOLUTION_DECISION_THRESHOLD";
    /// A9: Weighted sum of recent meaningful unprocessed decisions must reach this (default 3). See `growth_schedule`.
    pub const SKILLLITE_EVO_TRIGGER_WEIGHTED_MIN: &str = "SKILLLITE_EVO_TRIGGER_WEIGHTED_MIN";
    /// A9: Sliding window size for weighted trigger (default 10).
    pub const SKILLLITE_EVO_TRIGGER_SIGNAL_WINDOW: &str = "SKILLLITE_EVO_TRIGGER_SIGNAL_WINDOW";
    /// A9: If no **material** `evolution_run` (`evolution_log.type = evolution_run`, not `evolution_run_noop`) for this many seconds and weighted sum ≥ 1, allow sweep trigger (default 86400).
    pub const SKILLLITE_EVO_SWEEP_INTERVAL_SECS: &str = "SKILLLITE_EVO_SWEEP_INTERVAL_SECS";
    /// A9: Minimum seconds since last **material** `evolution_run` before another autorun (0 = disabled; `evolution_run_noop` ignored).
    pub const SKILLLITE_EVO_MIN_RUN_GAP_SEC: &str = "SKILLLITE_EVO_MIN_RUN_GAP_SEC";
    /// Skip snapshot + learners when there is no decision backlog (default on). Set `0` to disable.
    pub const SKILLLITE_EVO_SHALLOW_PREFLIGHT: &str = "SKILLLITE_EVO_SHALLOW_PREFLIGHT";
    /// Active-scope proposals: minimum stable successful decisions before active evolution (default 10).
    pub const SKILLLITE_EVO_ACTIVE_MIN_STABLE_DECISIONS: &str =
        "SKILLLITE_EVO_ACTIVE_MIN_STABLE_DECISIONS";
    /// Prompt snapshot dirs under `chat/prompts/_versions/` to keep after each evolution (oldest pruned first).
    /// Default `10`. Set to `0` to never delete snapshots (full local history, no Git required; disk usage grows).
    pub const SKILLLITE_EVOLUTION_SNAPSHOT_KEEP: &str = "SKILLLITE_EVOLUTION_SNAPSHOT_KEEP";
    /// Allow coordinator to auto-execute low-risk proposals when policy runtime is enabled.
    /// Default enabled (`1`/`true`).
    pub const SKILLLITE_EVO_AUTO_EXECUTE_LOW_RISK: &str = "SKILLLITE_EVO_AUTO_EXECUTE_LOW_RISK";
    /// Enable policy runtime in coordinator (`allow`/`ask`/`deny` with reason chain).
    /// Default enabled (`1`/`true`).
    pub const SKILLLITE_EVO_POLICY_RUNTIME_ENABLED: &str = "SKILLLITE_EVO_POLICY_RUNTIME_ENABLED";
    /// Deny critical-risk proposals in policy runtime by default.
    /// Default enabled (`1`/`true`).
    pub const SKILLLITE_EVO_DENY_CRITICAL: &str = "SKILLLITE_EVO_DENY_CRITICAL";
    /// Daily auto-execution budget for low-risk proposals (coordinator policy runtime).
    /// Default `5`.
    pub const SKILLLITE_EVO_RISK_BUDGET_LOW_PER_DAY: &str = "SKILLLITE_EVO_RISK_BUDGET_LOW_PER_DAY";
    /// Daily auto-execution budget for medium-risk proposals (default `0` = manual only).
    pub const SKILLLITE_EVO_RISK_BUDGET_MEDIUM_PER_DAY: &str =
        "SKILLLITE_EVO_RISK_BUDGET_MEDIUM_PER_DAY";
    /// Daily auto-execution budget for high-risk proposals (default `0` = manual only).
    pub const SKILLLITE_EVO_RISK_BUDGET_HIGH_PER_DAY: &str =
        "SKILLLITE_EVO_RISK_BUDGET_HIGH_PER_DAY";
    /// Daily auto-execution budget for critical-risk proposals (default `0`).
    pub const SKILLLITE_EVO_RISK_BUDGET_CRITICAL_PER_DAY: &str =
        "SKILLLITE_EVO_RISK_BUDGET_CRITICAL_PER_DAY";
    /// 进化触发场景：demo（更频繁）/ default（与不设一致）/ conservative（更少、省成本）。不设或 default 时行为与原有默认完全一致。
    pub const SKILLLITE_EVO_PROFILE: &str = "SKILLLITE_EVO_PROFILE";

    // ── 5.2 进化触发条件（高级，可单独覆盖；未设时由 EVO_PROFILE 或默认值决定）────────────
    /// 上次进化后冷却时间（小时），此时间内不再次触发。默认 0.5。
    pub const SKILLLITE_EVO_COOLDOWN_HOURS: &str = "SKILLLITE_EVO_COOLDOWN_HOURS";
    /// 统计决策的时间窗口（天）。默认 7。
    pub const SKILLLITE_EVO_RECENT_DAYS: &str = "SKILLLITE_EVO_RECENT_DAYS";
    /// 时间窗口内最多取多少条决策参与统计。默认 100。
    pub const SKILLLITE_EVO_RECENT_LIMIT: &str = "SKILLLITE_EVO_RECENT_LIMIT";
    /// 单条决策至少多少 tool 调用才计入「有意义」条数。默认 2。
    pub const SKILLLITE_EVO_MEANINGFUL_MIN_TOOLS: &str = "SKILLLITE_EVO_MEANINGFUL_MIN_TOOLS";
    /// 技能进化：有意义决策数 ≥ 此值且（有失败或存在重复模式）才触发。默认 3。
    pub const SKILLLITE_EVO_MEANINGFUL_THRESHOLD_SKILLS: &str =
        "SKILLLITE_EVO_MEANINGFUL_THRESHOLD_SKILLS";
    /// 记忆进化：有意义决策数 ≥ 此值才触发。默认 3。
    pub const SKILLLITE_EVO_MEANINGFUL_THRESHOLD_MEMORY: &str =
        "SKILLLITE_EVO_MEANINGFUL_THRESHOLD_MEMORY";
    /// 规则进化：有意义决策数 ≥ 此值且（失败次数或重规划次数达标）才触发。默认 5。
    pub const SKILLLITE_EVO_MEANINGFUL_THRESHOLD_PROMPTS: &str =
        "SKILLLITE_EVO_MEANINGFUL_THRESHOLD_PROMPTS";
    /// 规则进化：失败次数 ≥ 此值才考虑规则进化。默认 2。
    pub const SKILLLITE_EVO_FAILURES_MIN_PROMPTS: &str = "SKILLLITE_EVO_FAILURES_MIN_PROMPTS";
    /// 规则进化：重规划次数 ≥ 此值才考虑规则进化。默认 2。
    pub const SKILLLITE_EVO_REPLANS_MIN_PROMPTS: &str = "SKILLLITE_EVO_REPLANS_MIN_PROMPTS";
    /// 重复模式判定：同一模式出现次数 ≥ 此值且成功率达标才计为 repeated_pattern。默认 3。
    pub const SKILLLITE_EVO_REPEATED_PATTERN_MIN_COUNT: &str =
        "SKILLLITE_EVO_REPEATED_PATTERN_MIN_COUNT";
    /// 重复模式判定：成功率 ≥ 此值（0~1）。默认 0.8。
    pub const SKILLLITE_EVO_REPEATED_PATTERN_MIN_SUCCESS_RATE: &str =
        "SKILLLITE_EVO_REPEATED_PATTERN_MIN_SUCCESS_RATE";

    // ── Learner input windows (advanced; raise recall for rules/examples/memory/skills) ─────────
    /// Prompt learner: min `total_tools` for **example** generation candidate rows. Default `2`; set `1` or `3` to tune recall vs. signal strength.
    pub const SKILLLITE_EVO_PROMPT_EXAMPLE_MIN_TOOLS: &str =
        "SKILLLITE_EVO_PROMPT_EXAMPLE_MIN_TOOLS";
    /// Prompt learner: max recent decisions per success/fail bucket for **rule extraction** summaries. Default `10`.
    pub const SKILLLITE_EVO_PROMPT_RULE_SUMMARY_LIMIT: &str =
        "SKILLLITE_EVO_PROMPT_RULE_SUMMARY_LIMIT";
    /// Memory learner: lookback window in days for decision rows fed to extraction. Default `7`.
    pub const SKILLLITE_EVO_MEMORY_RECENT_DAYS: &str = "SKILLLITE_EVO_MEMORY_RECENT_DAYS";
    /// Memory learner: max decision rows in that window for the extraction prompt. Default `15`.
    pub const SKILLLITE_EVO_MEMORY_DECISION_LIMIT: &str = "SKILLLITE_EVO_MEMORY_DECISION_LIMIT";
    /// Skill synth DB queries: lookback days for pattern / failure queries. Default `7`.
    pub const SKILLLITE_EVO_SKILL_QUERY_RECENT_DAYS: &str = "SKILLLITE_EVO_SKILL_QUERY_RECENT_DAYS";
    /// Skill synth DB queries: row cap for recent decision scans (patterns, failures). Default `100`.
    pub const SKILLLITE_EVO_SKILL_QUERY_DECISION_LIMIT: &str =
        "SKILLLITE_EVO_SKILL_QUERY_DECISION_LIMIT";
    /// Skill synth: max rows when sampling per-skill failure context. Default `5`.
    pub const SKILLLITE_EVO_SKILL_FAILURE_SAMPLE_LIMIT: &str =
        "SKILLLITE_EVO_SKILL_FAILURE_SAMPLE_LIMIT";

    /// Acceptance window size (days) for auto-linking backlog acceptance status. Default 3.
    pub const SKILLLITE_EVO_ACCEPTANCE_WINDOW_DAYS: &str = "SKILLLITE_EVO_ACCEPTANCE_WINDOW_DAYS";
    /// Acceptance threshold: minimum first_success_rate in window. Default 0.70.
    pub const SKILLLITE_EVO_ACCEPTANCE_MIN_SUCCESS_RATE: &str =
        "SKILLLITE_EVO_ACCEPTANCE_MIN_SUCCESS_RATE";
    /// Acceptance threshold: maximum user_correction_rate in window. Default 0.20.
    pub const SKILLLITE_EVO_ACCEPTANCE_MAX_CORRECTION_RATE: &str =
        "SKILLLITE_EVO_ACCEPTANCE_MAX_CORRECTION_RATE";
    /// Acceptance threshold: maximum rollback_rate in window. Default 0.20.
    pub const SKILLLITE_EVO_ACCEPTANCE_MAX_ROLLBACK_RATE: &str =
        "SKILLLITE_EVO_ACCEPTANCE_MAX_ROLLBACK_RATE";

    /// Force a specific proposal id during evolution dry-run / debug.
    pub const SKILLLITE_EVO_FORCE_PROPOSAL_ID: &str = "SKILLLITE_EVO_FORCE_PROPOSAL_ID";

    /// Set to `1` to enable description-level dedup when synthesising new skills.
    pub const SKILLLITE_SKILL_DEDUP_DESCRIPTION: &str = "SKILLLITE_SKILL_DEDUP_DESCRIPTION";

    /// Set to `1` to allow ingesting external (community) learners.
    pub const SKILLLITE_EXTERNAL_LEARNING: &str = "SKILLLITE_EXTERNAL_LEARNING";

    /// Minimum recurrence count for a tool/argument pattern to be considered
    /// a candidate for skill synthesis.
    pub const SKILLLITE_MIN_PATTERN_COUNT: &str = "SKILLLITE_MIN_PATTERN_COUNT";
}

/// P2P swarm HTTP API (`skilllite swarm`) 与 `delegate_to_swarm`
pub mod swarm {
    /// 本机或远程 swarm 基址，例如 `http://127.0.0.1:7700`（`delegate_to_swarm` 使用）
    pub const SKILLLITE_SWARM_URL: &str = "SKILLLITE_SWARM_URL";
    /// 非空时：swarm HTTP 接口要求 `Authorization: Bearer <token>`；所有节点与调用方须配置相同值
    pub const SKILLLITE_SWARM_TOKEN: &str = "SKILLLITE_SWARM_TOKEN";
    /// 设为 `0` 时关闭 swarm handler 内的 LLM 路由决策，回退到静态规则。
    pub const SKILLLITE_SWARM_LLM_ROUTING: &str = "SKILLLITE_SWARM_LLM_ROUTING";
}

/// Channel-serve (HTTP/SMS bridge) configuration
pub mod channel {
    /// Authorization gate for `skilllite channel-serve` (must be set to `1` to allow run).
    pub const SKILLLITE_CHANNEL_SERVE_ALLOW: &str = "SKILLLITE_CHANNEL_SERVE_ALLOW";
    /// Bind address for the channel HTTP listener (e.g. `127.0.0.1:7800`).
    pub const SKILLLITE_CHANNEL_HTTP_ADDR: &str = "SKILLLITE_CHANNEL_HTTP_ADDR";
    /// Set to `1` to allow channel-serve to start without an auth token (insecure dev only).
    pub const SKILLLITE_CHANNEL_HTTP_ALLOW_INSECURE_NO_AUTH: &str =
        "SKILLLITE_CHANNEL_HTTP_ALLOW_INSECURE_NO_AUTH";
    /// DingTalk webhook URL for the DingTalk channel binding.
    pub const SKILLLITE_CHANNEL_DINGTALK_WEBHOOK: &str = "SKILLLITE_CHANNEL_DINGTALK_WEBHOOK";
    /// Optional DingTalk signing secret paired with the webhook.
    pub const SKILLLITE_CHANNEL_DINGTALK_SECRET: &str = "SKILLLITE_CHANNEL_DINGTALK_SECRET";
}

/// Artifact HTTP server (`skilllite artifact serve`)
pub mod artifact {
    /// Bind address for the artifact HTTP listener (e.g. `127.0.0.1:7900`).
    pub const SKILLLITE_ARTIFACT_HTTP_ADDR: &str = "SKILLLITE_ARTIFACT_HTTP_ADDR";
    /// Authorization gate to require auth tokens for the artifact server.
    pub const SKILLLITE_ARTIFACT_HTTP_REQUIRE_AUTH: &str = "SKILLLITE_ARTIFACT_HTTP_REQUIRE_AUTH";
    /// Explicit opt-in to start the artifact server without auth (insecure dev only).
    pub const SKILLLITE_ARTIFACT_HTTP_ALLOW_INSECURE_NO_AUTH: &str =
        "SKILLLITE_ARTIFACT_HTTP_ALLOW_INSECURE_NO_AUTH";
}

/// Executor / transcript flushing tunables
pub mod executor {
    /// Transcript flush mode: `every`/`interval`/`hybrid` (see `executor::transcript`).
    pub const SKILLLITE_TRANSCRIPT_FLUSH_MODE: &str = "SKILLLITE_TRANSCRIPT_FLUSH_MODE";
    /// Flush every N events (for `every` and `hybrid` modes).
    pub const SKILLLITE_TRANSCRIPT_FLUSH_EVERY: &str = "SKILLLITE_TRANSCRIPT_FLUSH_EVERY";
    /// Flush after this many milliseconds (for `interval` and `hybrid` modes).
    pub const SKILLLITE_TRANSCRIPT_FLUSH_INTERVAL_MS: &str =
        "SKILLLITE_TRANSCRIPT_FLUSH_INTERVAL_MS";
}

/// CLI command-layer overrides (used by `skilllite-commands`)
pub mod commands {
    /// Set to `1` to bypass the trust-confirmation prompt during `skill execute`.
    pub const SKILLLITE_TRUST_BYPASS_CONFIRM: &str = "SKILLLITE_TRUST_BYPASS_CONFIRM";
}

/// Desktop assistant (`skilllite-assistant`) — Tauri integration knobs
pub mod desktop {
    /// Heartbeat / life-pulse refresh interval (seconds).
    pub const SKILLLITE_HEARTBEAT_INTERVAL_SECS: &str = "SKILLLITE_HEARTBEAT_INTERVAL_SECS";
    /// Internal flag the desktop sets to authorize the gateway-serve subcommand.
    pub const SKILLLITE_GATEWAY_SERVE_ALLOW: &str = "SKILLLITE_GATEWAY_SERVE_ALLOW";
}

/// `skilllite-fs` mirror.
///
/// **Mirrored** because `skilllite-fs` is a leaf crate that `skilllite-core`
/// depends on; it cannot import from `skilllite-core::config::env_keys`.
/// The literal in `skilllite-fs/src/search_replace.rs` (currently encapsulated
/// in `skilllite_fs::env_keys::SKILLLITE_FUZZY_THRESHOLD`) MUST equal this
/// string. The `all_skilllite_env_literals_are_registered` consistency test
/// scans both crates and would fail on drift.
pub mod fs {
    pub const SKILLLITE_FUZZY_THRESHOLD: &str = "SKILLLITE_FUZZY_THRESHOLD";
}

/// A11: 高危工具确认 — 可配置哪些操作需发消息确认
pub mod high_risk {
    /// SKILLLITE_HIGH_RISK_CONFIRM: 逗号分隔，如 "write_key_path,run_command"；需要网络 skill 单独确认时加上 `network`。
    /// 可选值: write_key_path, run_command, network。默认 "write_key_path,run_command"（不含 network）。`all` 表示三项全开。
    /// "none" 表示全部跳过确认；"all" 等同默认。
    pub const SKILLLITE_HIGH_RISK_CONFIRM: &str = "SKILLLITE_HIGH_RISK_CONFIRM";
}

/// Agent 主循环：外层迭代上限与单任务工具调用预算
pub mod agent_loop {
    /// 全局迭代上限（默认 50）。有任务计划时，实际轮次还会与 `SKILLLITE_MAX_TOOL_CALLS_PER_TASK` 组合 capped。
    pub const SKILLLITE_MAX_ITERATIONS: &str = "SKILLLITE_MAX_ITERATIONS";
    /// 单任务内工具调用深度上限，并参与有计划时的有效迭代上限计算（默认 15）。
    pub const SKILLLITE_MAX_TOOL_CALLS_PER_TASK: &str = "SKILLLITE_MAX_TOOL_CALLS_PER_TASK";
}

/// 规划与 dependency-audit
pub mod misc {
    pub const SKILLLITE_COMPACT_PLANNING: &str = "SKILLLITE_COMPACT_PLANNING";
    pub const SKILLLITE_AUDIT_API: &str = "SKILLLITE_AUDIT_API";
    pub const PYPI_MIRROR_URL: &str = "PYPI_MIRROR_URL";
    pub const OSV_API_URL: &str = "OSV_API_URL";
}

/// 沙箱执行：级别、资源限制、开关等（SKILLLITE_* 优先，兼容 SKILLBOX_*）
pub mod sandbox {
    pub const SKILLLITE_SANDBOX_LEVEL: &str = "SKILLLITE_SANDBOX_LEVEL";
    pub const SANDBOX_LEVEL_ALIASES: &[&str] = &["SKILLBOX_SANDBOX_LEVEL"];

    pub const SKILLLITE_MAX_MEMORY_MB: &str = "SKILLLITE_MAX_MEMORY_MB";
    pub const MAX_MEMORY_MB_ALIASES: &[&str] = &["SKILLBOX_MAX_MEMORY_MB"];

    pub const SKILLLITE_TIMEOUT_SECS: &str = "SKILLLITE_TIMEOUT_SECS";
    pub const TIMEOUT_SECS_ALIASES: &[&str] = &["SKILLBOX_TIMEOUT_SECS"];

    pub const SKILLLITE_AUTO_APPROVE: &str = "SKILLLITE_AUTO_APPROVE";
    pub const AUTO_APPROVE_ALIASES: &[&str] = &["SKILLBOX_AUTO_APPROVE"];

    pub const SKILLLITE_NO_SANDBOX: &str = "SKILLLITE_NO_SANDBOX";
    pub const NO_SANDBOX_ALIASES: &[&str] = &["SKILLBOX_NO_SANDBOX"];

    /// When bwrap/firejail fail or are missing, allow weak PID/UTS/net namespace fallback (Linux only).
    pub const SKILLLITE_ALLOW_LINUX_NAMESPACE_FALLBACK: &str =
        "SKILLLITE_ALLOW_LINUX_NAMESPACE_FALLBACK";
    pub const ALLOW_LINUX_NAMESPACE_FALLBACK_ALIASES: &[&str] =
        &["SKILLBOX_ALLOW_LINUX_NAMESPACE_FALLBACK"];

    pub const SKILLLITE_ALLOW_PLAYWRIGHT: &str = "SKILLLITE_ALLOW_PLAYWRIGHT";
    pub const ALLOW_PLAYWRIGHT_ALIASES: &[&str] = &["SKILLBOX_ALLOW_PLAYWRIGHT"];

    pub const SKILLLITE_SCRIPT_ARGS: &str = "SKILLLITE_SCRIPT_ARGS";
    pub const SCRIPT_ARGS_ALIASES: &[&str] = &["SKILLBOX_SCRIPT_ARGS"];

    /// Cap on number of child processes the sandbox launcher will allow.
    pub const SKILLLITE_MAX_PROCESSES: &str = "SKILLLITE_MAX_PROCESSES";

    /// Set to `1` to skip the interactive runtime-dependency confirmation.
    pub const SKILLLITE_AUTO_APPROVE_RUNTIME: &str = "SKILLLITE_AUTO_APPROVE_RUNTIME";

    /// Optional mirror base URL for downloading the Python runtime.
    pub const SKILLLITE_RUNTIME_PYTHON_BASE_URL: &str = "SKILLLITE_RUNTIME_PYTHON_BASE_URL";

    /// Optional mirror base URL for downloading the Node.js runtime.
    pub const SKILLLITE_RUNTIME_NODE_BASE_URL: &str = "SKILLLITE_RUNTIME_NODE_BASE_URL";
}

/// Canonical, deduplicated, lexicographically-sorted list of every
/// `SKILLLITE_*` environment-variable name registered above.
///
/// Used by the `all_skilllite_env_literals_are_registered` consistency test
/// to enforce that no production code embeds a `SKILLLITE_*` literal that
/// is not in this registry.
///
/// **When adding a new `SKILLLITE_*` constant in this file, also add it
/// here.** The test will fail on any unregistered literal in workspace
/// production code, and a `debug_assert_sorted_unique` in this file's tests
/// fails if this list itself is not sorted/unique.
pub fn all_known_keys() -> &'static [&'static str] {
    // Keep strictly ASCII-sorted (note: '_' (0x5F) sorts AFTER ASCII letters).
    // Checked by `all_known_keys_sorted_and_unique`.
    &[
        "SKILLLITE_AGENT_MCP_CLIENT",
        "SKILLLITE_ALLOW_LINUX_NAMESPACE_FALLBACK",
        "SKILLLITE_ALLOW_PLAYWRIGHT",
        "SKILLLITE_API_BASE",
        "SKILLLITE_API_KEY",
        "SKILLLITE_ARTIFACT_HTTP_ADDR",
        "SKILLLITE_ARTIFACT_HTTP_ALLOW_INSECURE_NO_AUTH",
        "SKILLLITE_ARTIFACT_HTTP_REQUIRE_AUTH",
        "SKILLLITE_AUDIT_ALERT_EDIT_UNIQUE_PATHS",
        "SKILLLITE_AUDIT_ALERT_FAILURE_RATIO",
        "SKILLLITE_AUDIT_ALERT_MAX_INVOCATIONS_PER_SKILL",
        "SKILLLITE_AUDIT_ALERT_MIN_INVOCATIONS_FOR_FAILURE",
        "SKILLLITE_AUDIT_ALERT_WEBHOOK",
        "SKILLLITE_AUDIT_API",
        "SKILLLITE_AUDIT_CONTEXT",
        "SKILLLITE_AUDIT_DISABLED",
        "SKILLLITE_AUDIT_LOG",
        "SKILLLITE_AUTO_APPROVE",
        "SKILLLITE_AUTO_APPROVE_RUNTIME",
        "SKILLLITE_CACHE_DIR",
        "SKILLLITE_CHANNEL_DINGTALK_SECRET",
        "SKILLLITE_CHANNEL_DINGTALK_WEBHOOK",
        "SKILLLITE_CHANNEL_HTTP_ADDR",
        "SKILLLITE_CHANNEL_HTTP_ALLOW_INSECURE_NO_AUTH",
        "SKILLLITE_CHANNEL_SERVE_ALLOW",
        "SKILLLITE_CHUNK_SIZE",
        "SKILLLITE_COMPACTION_KEEP_RECENT",
        "SKILLLITE_COMPACTION_THRESHOLD",
        "SKILLLITE_COMPACT_PLANNING",
        "SKILLLITE_CONTEXT_SOFT_LIMIT_CHARS",
        "SKILLLITE_EMBEDDING_API_KEY",
        "SKILLLITE_EMBEDDING_BASE_URL",
        "SKILLLITE_EMBEDDING_DIMENSION",
        "SKILLLITE_EMBEDDING_MODEL",
        "SKILLLITE_ENABLE_MEMORY",
        "SKILLLITE_ENABLE_MEMORY_VECTOR",
        "SKILLLITE_ENABLE_TASK_PLANNING",
        "SKILLLITE_EVOLUTION",
        "SKILLLITE_EVOLUTION_DECISION_THRESHOLD",
        "SKILLLITE_EVOLUTION_INTERVAL_SECS",
        "SKILLLITE_EVOLUTION_SNAPSHOT_KEEP",
        "SKILLLITE_EVO_ACCEPTANCE_MAX_CORRECTION_RATE",
        "SKILLLITE_EVO_ACCEPTANCE_MAX_ROLLBACK_RATE",
        "SKILLLITE_EVO_ACCEPTANCE_MIN_SUCCESS_RATE",
        "SKILLLITE_EVO_ACCEPTANCE_WINDOW_DAYS",
        "SKILLLITE_EVO_ACTIVE_MIN_STABLE_DECISIONS",
        "SKILLLITE_EVO_AUTO_EXECUTE_LOW_RISK",
        "SKILLLITE_EVO_COOLDOWN_HOURS",
        "SKILLLITE_EVO_DENY_CRITICAL",
        "SKILLLITE_EVO_FAILURES_MIN_PROMPTS",
        "SKILLLITE_EVO_FORCE_PROPOSAL_ID",
        "SKILLLITE_EVO_MEANINGFUL_MIN_TOOLS",
        "SKILLLITE_EVO_MEANINGFUL_THRESHOLD_MEMORY",
        "SKILLLITE_EVO_MEANINGFUL_THRESHOLD_PROMPTS",
        "SKILLLITE_EVO_MEANINGFUL_THRESHOLD_SKILLS",
        "SKILLLITE_EVO_MEMORY_DECISION_LIMIT",
        "SKILLLITE_EVO_MEMORY_RECENT_DAYS",
        "SKILLLITE_EVO_MIN_RUN_GAP_SEC",
        "SKILLLITE_EVO_POLICY_RUNTIME_ENABLED",
        "SKILLLITE_EVO_PROFILE",
        "SKILLLITE_EVO_PROMPT_EXAMPLE_MIN_TOOLS",
        "SKILLLITE_EVO_PROMPT_RULE_SUMMARY_LIMIT",
        "SKILLLITE_EVO_RECENT_DAYS",
        "SKILLLITE_EVO_RECENT_LIMIT",
        "SKILLLITE_EVO_REPEATED_PATTERN_MIN_COUNT",
        "SKILLLITE_EVO_REPEATED_PATTERN_MIN_SUCCESS_RATE",
        "SKILLLITE_EVO_REPLANS_MIN_PROMPTS",
        "SKILLLITE_EVO_RISK_BUDGET_CRITICAL_PER_DAY",
        "SKILLLITE_EVO_RISK_BUDGET_HIGH_PER_DAY",
        "SKILLLITE_EVO_RISK_BUDGET_LOW_PER_DAY",
        "SKILLLITE_EVO_RISK_BUDGET_MEDIUM_PER_DAY",
        "SKILLLITE_EVO_SHALLOW_PREFLIGHT",
        "SKILLLITE_EVO_SKILL_FAILURE_SAMPLE_LIMIT",
        "SKILLLITE_EVO_SKILL_QUERY_DECISION_LIMIT",
        "SKILLLITE_EVO_SKILL_QUERY_RECENT_DAYS",
        "SKILLLITE_EVO_SWEEP_INTERVAL_SECS",
        "SKILLLITE_EVO_TRIGGER_SIGNAL_WINDOW",
        "SKILLLITE_EVO_TRIGGER_WEIGHTED_MIN",
        "SKILLLITE_EXTERNAL_LEARNING",
        "SKILLLITE_EXTRACT_TOP_K_RATIO",
        "SKILLLITE_FUZZY_THRESHOLD",
        "SKILLLITE_GATEWAY_SERVE_ALLOW",
        "SKILLLITE_GOAL_LLM_EXTRACT",
        "SKILLLITE_HEAD_CHUNKS",
        "SKILLLITE_HEARTBEAT_INTERVAL_SECS",
        "SKILLLITE_HIGH_RISK_CONFIRM",
        "SKILLLITE_HISTORY_WINDOW_MESSAGES",
        "SKILLLITE_LOG_JSON",
        "SKILLLITE_LOG_LEVEL",
        "SKILLLITE_LONG_TEXT_STRATEGY",
        "SKILLLITE_MAP_MODEL",
        "SKILLLITE_MAX_EVOLUTIONS_PER_DAY",
        "SKILLLITE_MAX_ITERATIONS",
        "SKILLLITE_MAX_MEMORY_MB",
        "SKILLLITE_MAX_OUTPUT_CHARS",
        "SKILLLITE_MAX_PROCESSES",
        "SKILLLITE_MAX_TOKENS",
        "SKILLLITE_MAX_TOOL_CALLS_PER_TASK",
        "SKILLLITE_MCP_SERVERS_JSON",
        "SKILLLITE_MEMORY_FLUSH_ENABLED",
        "SKILLLITE_MEMORY_FLUSH_THRESHOLD",
        "SKILLLITE_MIN_PATTERN_COUNT",
        "SKILLLITE_MODEL",
        "SKILLLITE_NETWORK_DISABLED",
        "SKILLLITE_NO_SANDBOX",
        "SKILLLITE_OUTPUT_DIR",
        "SKILLLITE_QUIET",
        "SKILLLITE_READ_FILE_TOOL_RESULT_MAX_CHARS",
        "SKILLLITE_RUNTIME_NODE_BASE_URL",
        "SKILLLITE_RUNTIME_PYTHON_BASE_URL",
        "SKILLLITE_SANDBOX",
        "SKILLLITE_SANDBOX_LEVEL",
        "SKILLLITE_SCHEDULE_ENABLED",
        "SKILLLITE_SCRIPT_ARGS",
        "SKILLLITE_SECURITY_EVENTS_LOG",
        "SKILLLITE_SKILLS_DIR",
        "SKILLLITE_SKILLS_REPO",
        "SKILLLITE_SKILL_DEDUP_DESCRIPTION",
        "SKILLLITE_SKILL_DENYLIST",
        "SKILLLITE_SUMMARIZE_THRESHOLD",
        "SKILLLITE_SUPPLY_CHAIN_BLOCK",
        "SKILLLITE_SWARM_LLM_ROUTING",
        "SKILLLITE_SWARM_TOKEN",
        "SKILLLITE_SWARM_URL",
        "SKILLLITE_TAIL_CHUNKS",
        "SKILLLITE_TIMEOUT_SECS",
        "SKILLLITE_TOOL_RESULT_MAX_CHARS",
        "SKILLLITE_TOOL_RESULT_RECOVERY_MAX_CHARS",
        "SKILLLITE_TRANSCRIPT_FLUSH_EVERY",
        "SKILLLITE_TRANSCRIPT_FLUSH_INTERVAL_MS",
        "SKILLLITE_TRANSCRIPT_FLUSH_MODE",
        "SKILLLITE_TRUST_BYPASS_CONFIRM",
        "SKILLLITE_UI_LOCALE",
        "SKILLLITE_USER_INPUT_MAX_CHARS",
        "SKILLLITE_WORKSPACE",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `all_known_keys()` must stay sorted (alphabetical) and free of
    /// duplicates so the consistency-test set semantics is well-defined.
    #[test]
    fn all_known_keys_sorted_and_unique() {
        let keys = all_known_keys();
        for window in keys.windows(2) {
            assert!(
                window[0] < window[1],
                "all_known_keys must be strictly sorted; offending pair: {:?} >= {:?}",
                window[0],
                window[1]
            );
        }
    }

    /// Sanity: every key must look like a `SKILLLITE_*` env-var name.
    #[test]
    fn all_known_keys_have_skilllite_prefix() {
        for key in all_known_keys() {
            assert!(
                key.starts_with("SKILLLITE_"),
                "{} is not a SKILLLITE_* env var",
                key
            );
            assert!(
                key.chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'),
                "{} contains characters that are not [A-Z0-9_]",
                key
            );
        }
    }

    /// Sanity: a few well-known keys really resolve through the per-domain
    /// `pub const` they correspond to. Catches accidental rename / typo on
    /// the manual `all_known_keys()` slice.
    #[test]
    fn registry_matches_known_modules() {
        let keys = all_known_keys();
        let must_contain = [
            llm::API_BASE,
            llm::API_KEY,
            llm::MODEL,
            paths::SKILLLITE_SKILLS_DIR,
            paths::SKILLLITE_SANDBOX,
            paths::SKILLLITE_NETWORK_DISABLED,
            agent_loop::SKILLLITE_MAX_ITERATIONS,
            agent_loop::SKILLLITE_MAX_TOOL_CALLS_PER_TASK,
            sandbox::SKILLLITE_SANDBOX_LEVEL,
            sandbox::SKILLLITE_MAX_PROCESSES,
            sandbox::SKILLLITE_AUTO_APPROVE_RUNTIME,
            sandbox::SKILLLITE_RUNTIME_PYTHON_BASE_URL,
            sandbox::SKILLLITE_RUNTIME_NODE_BASE_URL,
            evolution::SKILLLITE_EVOLUTION,
            evolution::SKILLLITE_EVO_FORCE_PROPOSAL_ID,
            evolution::SKILLLITE_SKILL_DEDUP_DESCRIPTION,
            evolution::SKILLLITE_EXTERNAL_LEARNING,
            evolution::SKILLLITE_MIN_PATTERN_COUNT,
            swarm::SKILLLITE_SWARM_URL,
            swarm::SKILLLITE_SWARM_LLM_ROUTING,
            channel::SKILLLITE_CHANNEL_SERVE_ALLOW,
            channel::SKILLLITE_CHANNEL_HTTP_ADDR,
            channel::SKILLLITE_CHANNEL_HTTP_ALLOW_INSECURE_NO_AUTH,
            channel::SKILLLITE_CHANNEL_DINGTALK_WEBHOOK,
            channel::SKILLLITE_CHANNEL_DINGTALK_SECRET,
            artifact::SKILLLITE_ARTIFACT_HTTP_ADDR,
            artifact::SKILLLITE_ARTIFACT_HTTP_REQUIRE_AUTH,
            artifact::SKILLLITE_ARTIFACT_HTTP_ALLOW_INSECURE_NO_AUTH,
            memory::SKILLLITE_EMBEDDING_BASE_URL,
            memory::SKILLLITE_EMBEDDING_API_KEY,
            memory::SKILLLITE_ENABLE_MEMORY,
            memory::SKILLLITE_ENABLE_MEMORY_VECTOR,
            executor::SKILLLITE_TRANSCRIPT_FLUSH_MODE,
            executor::SKILLLITE_TRANSCRIPT_FLUSH_EVERY,
            executor::SKILLLITE_TRANSCRIPT_FLUSH_INTERVAL_MS,
            commands::SKILLLITE_TRUST_BYPASS_CONFIRM,
            desktop::SKILLLITE_HEARTBEAT_INTERVAL_SECS,
            desktop::SKILLLITE_GATEWAY_SERVE_ALLOW,
            mcp::SKILLLITE_MCP_SERVERS_JSON,
            mcp::SKILLLITE_AGENT_MCP_CLIENT,
            goal::SKILLLITE_GOAL_LLM_EXTRACT,
            summarization::SKILLLITE_CHUNK_SIZE,
            summarization::SKILLLITE_HEAD_CHUNKS,
            summarization::SKILLLITE_TAIL_CHUNKS,
            summarization::SKILLLITE_MAX_OUTPUT_CHARS,
            summarization::SKILLLITE_MAP_MODEL,
            summarization::SKILLLITE_LONG_TEXT_STRATEGY,
            summarization::SKILLLITE_EXTRACT_TOP_K_RATIO,
            summarization::SKILLLITE_SUMMARIZE_THRESHOLD,
            summarization::SKILLLITE_MAX_TOKENS,
            summarization::SKILLLITE_USER_INPUT_MAX_CHARS,
            summarization::SKILLLITE_TOOL_RESULT_MAX_CHARS,
            summarization::SKILLLITE_READ_FILE_TOOL_RESULT_MAX_CHARS,
            summarization::SKILLLITE_TOOL_RESULT_RECOVERY_MAX_CHARS,
            summarization::SKILLLITE_CONTEXT_SOFT_LIMIT_CHARS,
            summarization::SKILLLITE_COMPACTION_THRESHOLD,
            summarization::SKILLLITE_MEMORY_FLUSH_ENABLED,
            summarization::SKILLLITE_MEMORY_FLUSH_THRESHOLD,
            summarization::SKILLLITE_COMPACTION_KEEP_RECENT,
            summarization::SKILLLITE_HISTORY_WINDOW_MESSAGES,
            fs::SKILLLITE_FUZZY_THRESHOLD,
        ];
        for k in must_contain {
            assert!(
                keys.binary_search(&k).is_ok(),
                "all_known_keys() missing {}",
                k
            );
        }
    }

    /// Walk every Rust source file under `crates/*/src/**/*.rs` (excluding
    /// this registry and test files), extract every `SKILLLITE_[A-Z0-9_]+`
    /// literal, and assert each is registered in `all_known_keys()`.
    ///
    /// This is the **falsifiability anchor** for the env-var single-source
    /// invariant: deleting any constant from `env_keys.rs` would leave its
    /// production literal orphaned, and this test would fail.
    #[test]
    fn all_skilllite_env_literals_are_registered() {
        let workspace_root = workspace_root_for_tests();
        let crates_dir = workspace_root.join("crates");
        assert!(
            crates_dir.is_dir(),
            "expected workspace crates dir at {:?} to exist",
            crates_dir
        );

        let registered: std::collections::HashSet<&'static str> =
            all_known_keys().iter().copied().collect();

        let mut offenders: Vec<(std::path::PathBuf, String, usize)> = Vec::new();
        let env_keys_path = std::path::PathBuf::from(file!());
        let env_keys_filename = env_keys_path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();

        walk_rust_sources(&crates_dir, &mut |path, contents| {
            // Skip the registry itself.
            if path.file_name().map(|s| s == env_keys_filename.as_str()) == Some(true) {
                return;
            }
            // Skip test-only files.
            if is_test_file_path(path) {
                return;
            }
            // Truncate at the first top-level `#[cfg(test)]` (column 0) — the
            // repo convention is to keep unit tests at the bottom of the file
            // inside `#[cfg(test)] mod tests { … }`. This avoids false
            // positives on test-only fixture names that intentionally use a
            // `SKILLLITE_*`-shaped string as a unique env key.
            let production_slice = production_prefix(contents);
            for (line_no, line) in production_slice.lines().enumerate() {
                for raw in extract_skilllite_literals(line) {
                    if !registered.contains(raw.as_str()) {
                        offenders.push((path.to_path_buf(), raw, line_no + 1));
                    }
                }
            }
        });

        assert!(
            offenders.is_empty(),
            "Found {} SKILLLITE_* literal(s) in production code that are not registered in env_keys::all_known_keys():\n{}",
            offenders.len(),
            offenders
                .iter()
                .map(|(p, name, line)| format!("  {} (line {}): {}", p.display(), line, name))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }

    fn workspace_root_for_tests() -> std::path::PathBuf {
        // CARGO_MANIFEST_DIR points to crates/skilllite-core; walk up to workspace root.
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .parent() // crates/
            .and_then(|p| p.parent()) // workspace root
            .map(|p| p.to_path_buf())
            .expect("expected workspace root above CARGO_MANIFEST_DIR")
    }

    /// Return the slice of `contents` before the first top-level
    /// `#[cfg(test)]` line (one starting at column 0). If no such line
    /// exists, returns the entire `contents`.
    fn production_prefix(contents: &str) -> &str {
        let needle = "#[cfg(test)]";
        // Look for `\n#[cfg(test)]` (line starting at column 0, after any line)
        // OR the file starting with it.
        if contents.starts_with(needle) {
            return "";
        }
        let pat = "\n#[cfg(test)]";
        match contents.find(pat) {
            Some(pos) => &contents[..pos],
            None => contents,
        }
    }

    fn is_test_file_path(path: &std::path::Path) -> bool {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if name == "tests.rs" || name.ends_with("_tests.rs") || name.ends_with("_test.rs") {
            return true;
        }
        path.components().any(|c| {
            matches!(
                c.as_os_str().to_str(),
                Some("tests") | Some("benches") | Some("examples")
            )
        })
    }

    fn walk_rust_sources(dir: &std::path::Path, visit: &mut dyn FnMut(&std::path::Path, &str)) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip `target/`, build artifacts, vendored deps, .git, etc.
                let name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                if matches!(name, "target" | "node_modules" | ".git" | "dist") {
                    continue;
                }
                walk_rust_sources(&path, visit);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(contents) = std::fs::read_to_string(&path) {
                    visit(&path, &contents);
                }
            }
        }
    }

    /// Falsifiability anchor for `all_skilllite_env_literals_are_registered`:
    /// directly verifies that the helper detects an unregistered name. If
    /// someone removes the registry-walk test or weakens it to vacuously
    /// pass, this still proves the detector itself is functional.
    #[test]
    fn extractor_detects_unregistered_literal() {
        let line = r#"let _ = std::env::var("SKILLLITE_DEFINITELY_NOT_REGISTERED_TEST_ONLY");"#;
        let hits = extract_skilllite_literals(line);
        assert_eq!(
            hits,
            vec!["SKILLLITE_DEFINITELY_NOT_REGISTERED_TEST_ONLY".to_string()],
            "extractor failed to find a SKILLLITE_* literal that the consistency test relies on"
        );
        let registered: std::collections::HashSet<&'static str> =
            all_known_keys().iter().copied().collect();
        assert!(
            !registered.contains("SKILLLITE_DEFINITELY_NOT_REGISTERED_TEST_ONLY"),
            "the synthetic unregistered name accidentally exists in the registry"
        );
    }

    /// `production_prefix` must truncate before the test mod and treat a
    /// file that starts with `#[cfg(test)]` as having no production lines.
    #[test]
    fn production_prefix_truncates_test_block() {
        let src = "fn foo() { let _ = \"SKILLLITE_X\"; }\n#[cfg(test)]\nmod t { fn bar() { let _ = \"SKILLLITE_TEST_ONLY\"; } }\n";
        let prefix = production_prefix(src);
        assert!(prefix.contains("SKILLLITE_X"));
        assert!(!prefix.contains("SKILLLITE_TEST_ONLY"));

        let test_only = "#[cfg(test)]\nmod t { fn bar() { let _ = \"SKILLLITE_TEST_ONLY\"; } }\n";
        assert_eq!(production_prefix(test_only), "");
    }

    /// Extract every `SKILLLITE_[A-Z0-9_]+` substring on a line.
    /// Liberal on context: it does not require a surrounding double-quote,
    /// which is intentional — we want to catch the name regardless of
    /// whether it appears as a string literal, a raw identifier,
    /// or inside a `cmd.env(…, …)` first arg. Lines starting with `//` or
    /// containing `// SKIP-ENV-AUDIT` are ignored to allow targeted opt-out.
    fn extract_skilllite_literals(line: &str) -> Vec<String> {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with("//!") {
            return Vec::new();
        }
        if line.contains("SKIP-ENV-AUDIT") {
            return Vec::new();
        }
        let bytes = line.as_bytes();
        let mut out = Vec::new();
        let mut i = 0;
        let needle = b"SKILLLITE_";
        while i + needle.len() <= bytes.len() {
            if &bytes[i..i + needle.len()] == needle {
                // Reject if preceded by an identifier char (so `MY_SKILLLITE_FOO`
                // wouldn't match — though no such name exists today).
                let prev_is_word =
                    i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
                if prev_is_word {
                    i += 1;
                    continue;
                }
                let mut end = i + needle.len();
                while end < bytes.len()
                    && (bytes[end].is_ascii_uppercase()
                        || bytes[end].is_ascii_digit()
                        || bytes[end] == b'_')
                {
                    end += 1;
                }
                let name = &line[i..end];
                // Reject lone `SKILLLITE_` (no suffix) — likely a prefix used in
                // `starts_with("SKILLLITE_")` check, not an env-var name.
                if end > i + needle.len() {
                    out.push(name.to_string());
                }
                i = end;
            } else {
                i += 1;
            }
        }
        out
    }
}
