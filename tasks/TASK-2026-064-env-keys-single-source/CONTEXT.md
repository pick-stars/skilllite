# Technical Context

## Current State

- Relevant crates/files:
  - `crates/skilllite-core/src/config/env_keys.rs` — canonical registry (~50
    constants). Today incomplete.
  - `crates/skilllite-core/src/config/loader.rs` — `env_optional(name, aliases)`
    helper. Some call sites pass a magic string for `name` instead of a
    `env_keys` constant.
  - `crates/skilllite-agent/src/types/env_config.rs` — defines local helpers
    (`env_usize`, `env_str`) that take a `&str` key. Each call inlines a
    `"SKILLLITE_…"` literal (~14 distinct vars).
  - `crates/skilllite-agent/src/agent_loop/helpers.rs:661` —
    `SKILLLITE_GOAL_LLM_EXTRACT`.
  - `crates/skilllite-agent/src/chat_session.rs:1064` —
    `SKILLLITE_HISTORY_WINDOW_MESSAGES`.
  - `crates/skilllite-agent/src/types/config.rs:121` —
    `SKILLLITE_MCP_SERVERS_JSON`.
  - `crates/skilllite-agent/src/mcp_client/bootstrap.rs:34` —
    `SKILLLITE_AGENT_MCP_CLIENT`.
  - `crates/skilllite-evolution/src/{config.rs, run.rs, external_learner.rs,
    skill_synth/mod.rs, skill_synth/generate.rs}` — five distinct vars.
  - `crates/skilllite-sandbox/src/{common.rs, env/runtime_deps.rs}` —
    `SKILLLITE_MAX_PROCESSES`, `SKILLLITE_AUTO_APPROVE_RUNTIME`,
    `SKILLLITE_RUNTIME_PYTHON_BASE_URL`, `SKILLLITE_RUNTIME_NODE_BASE_URL`.
  - `crates/skilllite-commands/src/{channel_serve.rs, execute.rs}` —
    channel/dingtalk envs and `SKILLLITE_TRUST_BYPASS_CONFIRM`.
  - `crates/skilllite-executor/src/transcript.rs` —
    `SKILLLITE_TRANSCRIPT_FLUSH_*` (3 vars).
  - `crates/skilllite-swarm/src/handler.rs:570` —
    `SKILLLITE_SWARM_LLM_ROUTING`.
  - `crates/skilllite-artifact/src/serve.rs:15,20` — already-declared local
    `pub const`s; should re-export from `env_keys` instead.
  - `crates/skilllite-fs/src/search_replace.rs:331` —
    `SKILLLITE_FUZZY_THRESHOLD`. Cannot import `env_keys` (layering).
  - `crates/skilllite-assistant/src-tauri/src/{life_pulse.rs, gateway_manager.rs,
    skilllite_bridge/{chat.rs, integrations/{evolution_ui.rs, skill_rpc.rs}}}`
    — write-side env_map manipulation that hard-codes the same names.
- Current behavior: All sites work correctly (the literals match the canonical
  names exactly today). The risk is silent drift on future edits, since
  there is no compile-time or test-time link between the literal and the
  registry.

## Architecture Fit

- Layer boundaries involved:
  - `skilllite-core` is the canonical owner of config/env constants. All
    higher-layer crates depend on it transitively (per `Cargo.toml`).
  - `skilllite-fs` is a leaf and `skilllite-core` depends on **it** (not the
    other way). Therefore `skilllite-fs` must keep its own local constant
    string. Test will still scan it.
- Interfaces to preserve:
  - `skilllite_core::config::loader::env_optional(name, aliases)` signature.
  - `env_keys` module hierarchy and existing public exports (additive only;
    no renames or removals).

## Dependency and Compatibility

- New dependencies: None. (Consistency test uses `std::fs` to walk source —
  no new crates.)
- Backward compatibility notes:
  - Adding `pub const` items is additive.
  - `all_known_keys()` is a new public function.
  - No env-var rename. No alias change. No default-value change.

## Design Decisions

- Decision: Add `env_keys::all_known_keys() -> &'static [&'static str]` as
  manually-maintained sorted slice rather than auto-derive.
  - Rationale: Manual list keeps the registry explicit and grep-friendly;
    a derive macro would add code-gen overhead for a one-shot test invariant.
  - Alternatives considered: `inventory` crate auto-collection.
  - Why rejected: Adds a runtime registration mechanism + dependency for a
    static sorted list that is small and rarely changes.

- Decision: Consistency test scans `crates/*/src/**/*.rs` only, excluding
  test files and `env_keys.rs` itself.
  - Rationale: Test files may legitimately reference future/experimental env
    names for fixture purposes; including them creates noise. The registry
    file is the source of literals — including it would short-circuit the test.
  - Alternatives considered: Scan everything; whitelist exceptions per name.
  - Why rejected: More fragile and harder to maintain.

- Decision: For `skilllite-fs`, use a file-local `pub(crate) const` whose
  string value equals the canonical entry. Add a `// Mirrored in skilllite-fs`
  comment in `env_keys.rs`.
  - Rationale: Cannot create `skilllite-fs → skilllite-core` dep; mirror is a
    documented one-line exception. The test still verifies the string equals
    the registered name.
  - Alternatives considered: Move the constant declaration into a 3rd shared
    crate that both can depend on.
  - Why rejected: Over-engineering for one variable; would require workspace
    layout change for marginal benefit.

- Decision: Refactor write-side too (`cmd.env(name, …)`,
  `m.insert(name, …)`, `set_var(name, …)`).
  - Rationale: A consistency test that only catches reads leaves write-side
    drift undetected. The test scans every `SKILLLITE_*` literal regardless
    of position, so writes must use the constants too.

## Open Questions

- [ ] (Resolved during implementation.) Whether to also walk
      `skilllite/src/**/*.rs` (the workspace `[[bin]]` package). → Yes,
      it is a production crate.
