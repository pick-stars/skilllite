# Status Journal

## Timeline

- 2026-04-29:
  - Progress: TASK / PRD / CONTEXT drafted. Initial audit identified ~21
    direct `env::var("SKILLLITE_…")` bypasses; broader scan via the new
    consistency test surfaced an additional ~26 indirect bypasses
    (`env_optional("SKILLLITE_…")`, `env_usize("SKILLLITE_…")`, `cmd.env`,
    `m.insert`) → ~47 distinct names total.
  - Implementation:
    1. Extended `crates/skilllite-core/src/config/env_keys.rs` with new
       `summarization`, `mcp`, `goal`, `channel`, `artifact`, `executor`,
       `commands`, `desktop`, `fs` modules and ~37 new `pub const` entries.
    2. Added `paths::SKILLLITE_SANDBOX` and `paths::SKILLLITE_NETWORK_DISABLED`
       (sandbox child-process markers).
    3. Extended `memory` module with `SKILLLITE_ENABLE_MEMORY`,
       `SKILLLITE_ENABLE_MEMORY_VECTOR`, `SKILLLITE_EMBEDDING_BASE_URL`,
       `SKILLLITE_EMBEDDING_API_KEY`.
    4. Added `all_known_keys() -> &'static [&'static str]` (sorted, unique)
       plus three structural tests
       (`all_known_keys_sorted_and_unique`, `all_known_keys_have_skilllite_prefix`,
       `registry_matches_known_modules`).
    5. Added `all_skilllite_env_literals_are_registered` consistency test:
       walks `crates/*/src/**/*.rs`, truncates each file at the first
       top-level `#[cfg(test)]`, extracts every `SKILLLITE_[A-Z0-9_]+`
       substring (with `// SKIP-ENV-AUDIT` opt-out), asserts each is in
       `all_known_keys()`. Helper unit tests
       (`production_prefix_truncates_test_block`,
       `extractor_detects_unregistered_literal`) prove the scanner is not
       vacuous.
    6. Refactored production read-sites in
       `skilllite-core` (`config/schema.rs`, `config/loader.rs` deprecated
       table), `skilllite-agent` (`agent_loop/helpers.rs`, `chat_session.rs`,
       `mcp_client/bootstrap.rs`, `types/config.rs`, `types/env_config.rs`,
       `skills/executor.rs`), `skilllite-evolution`
       (`config.rs`, `run.rs`, `external_learner.rs`, `skill_synth/mod.rs`,
       `skill_synth/generate.rs`), `skilllite-sandbox`
       (`common.rs`, `env/runtime_deps.rs`, `linux.rs`), `skilllite-executor`
       (`transcript.rs`), `skilllite-swarm` (`handler.rs`),
       `skilllite-commands` (`channel_serve.rs`, `execute.rs`, `ide.rs`),
       `skilllite-artifact` (`serve.rs`), `skilllite-assistant`
       (`life_pulse.rs`, `gateway_manager.rs`,
       `skilllite_bridge/{chat.rs, integrations/{evolution_ui.rs, skill_rpc.rs}}`).
       All `env::var(literal)`, `env_optional(literal)`,
       `cmd.env(literal, …)`, `set_var(literal, …)`, `m.insert(literal, …)`
       call sites now reference `env_keys::*` constants.
    7. Mirror: `skilllite-fs` (cannot depend on `skilllite-core` due to
       reverse layering) declares its own `pub const SKILLLITE_FUZZY_THRESHOLD`
       in a new `crates/skilllite-fs/src/env_keys.rs` module; its string value
       equals the canonical `env_keys::fs::SKILLLITE_FUZZY_THRESHOLD`. The
       consistency test scans `skilllite-fs` too, so any drift fails CI.
    8. Fixed stale error string `Invalid SKILLLITE_SKILLS_ROOT` →
       `Invalid SKILLLITE_SKILLS_DIR` in `skilllite-core/src/error.rs` (the
       referenced env var was renamed long ago).
    9. Documented the newly-registered vars in
       `docs/en/ENV_REFERENCE.md` and `docs/zh/ENV_REFERENCE.md`
       (`spec/docs-sync.md`).
    10. Added `// SKIP-ENV-AUDIT` opt-out on two prefix-match lines in
        `evolution_ui::is_evolution_runtime_env_key` (where
        `starts_with("SKILLLITE_EVO")` is intentionally a prefix, not a
        single env-var name).
  - Falsifiability check: temporarily removed
    `"SKILLLITE_FUZZY_THRESHOLD"` from `all_known_keys()`; both
    `registry_matches_known_modules` and
    `all_skilllite_env_literals_are_registered` failed loudly (one with
    `all_known_keys() missing SKILLLITE_FUZZY_THRESHOLD`, the other with
    `Found 1 SKILLLITE_* literal(s) … not registered: …/skilllite-fs/src/search_replace.rs (line 331): SKILLLITE_FUZZY_THRESHOLD`).
    Entry restored.
  - Validation:
    - `cargo fmt --check` (workspace + assistant manifest) → pass.
    - `cargo clippy --all-targets -- -D warnings` (default workspace,
      matches CI) → pass; no new warnings.
    - `cargo build --workspace --all-targets` → pass.
    - `cargo build --manifest-path crates/skilllite-assistant/src-tauri/Cargo.toml`
      → pass (one pre-existing `private_interfaces` warning in
      `gateway_manager.rs:27`, unrelated and untouched here).
    - `cargo test --workspace` → all green; representative counts:
      `skilllite-core 243 passed`, `skilllite-agent 86 passed`,
      `skilllite-sandbox 86 passed`, `skilllite-evolution 94 passed`,
      `skilllite-commands 28 passed`, `skilllite-swarm 12 passed`,
      `skilllite-fs 23 passed`, `skilllite-executor 20 passed`,
      `skilllite-artifact 17 passed`, etc. New env_keys tests included.
    - `cargo test -p skilllite-core --lib config::env_keys` → 6 tests
      pass (`all_known_keys_sorted_and_unique`,
      `all_known_keys_have_skilllite_prefix`,
      `registry_matches_known_modules`,
      `extractor_detects_unregistered_literal`,
      `production_prefix_truncates_test_block`,
      `all_skilllite_env_literals_are_registered`).
    - `cargo deny check bans` → `bans ok` (pre-existing
      `unused-wrapper` warnings from `deny.toml` unaffected).
    - `python3 scripts/validate_tasks.py` →
      `Task validation passed (64 task directories checked).`
    - Manual hunt:
      `rg 'env::var\("SKILLLITE_'  crates/*/src crates/skilllite-assistant/src-tauri/src skilllite/src`
      returns zero hits (after excluding `env_keys.rs`).
      `rg '(env_optional|env_or|env_bool|env_compat)\("SKILLLITE_'`
      similarly returns zero outside `env_keys.rs`.
  - Blockers: None.
  - Next step: Hand back to user; subsequent unrelated work continues
    from `TASK-2026-064` checkpoint.

## Checkpoints

- [x] PRD drafted before implementation (or `N/A` recorded)
- [x] Context drafted before implementation (or `N/A` recorded)
- [x] Implementation complete
- [x] Tests passed
- [x] Review complete
- [x] Board updated
