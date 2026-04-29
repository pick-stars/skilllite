# Review Report

## Scope Reviewed

- Files/modules:
  - `crates/skilllite-core/src/config/env_keys.rs` — registry +
    consistency test (~+580 lines, ~+37 new constants).
  - `crates/skilllite-core/src/config/loader.rs` — alias-table now
    references `env_keys::*` constants.
  - `crates/skilllite-core/src/config/schema.rs` — `AgentFeatureFlags::from_env`,
    `EmbeddingConfig::from_env` now use `env_keys::memory::*`.
  - `crates/skilllite-core/src/error.rs` — corrected stale
    `SKILLLITE_SKILLS_ROOT` → `SKILLLITE_SKILLS_DIR`.
  - `crates/skilllite-fs/src/{lib.rs, env_keys.rs, search_replace.rs}` —
    new local mirror module + reference; documented in core registry.
  - `crates/skilllite-agent/src/{agent_loop/helpers.rs, chat_session.rs,
    mcp_client/bootstrap.rs, types/{config.rs, env_config.rs}, skills/executor.rs}`.
  - `crates/skilllite-evolution/src/{config.rs, run.rs,
    external_learner.rs, skill_synth/mod.rs, skill_synth/generate.rs}`.
  - `crates/skilllite-sandbox/src/{common.rs, env/runtime_deps.rs, linux.rs}`.
  - `crates/skilllite-executor/src/transcript.rs`.
  - `crates/skilllite-swarm/src/handler.rs`.
  - `crates/skilllite-commands/src/{channel_serve.rs, execute.rs, ide.rs}`.
  - `crates/skilllite-artifact/src/serve.rs`.
  - `crates/skilllite-assistant/src-tauri/src/{life_pulse.rs, gateway_manager.rs,
    skilllite_bridge/{chat.rs, integrations/{evolution_ui.rs, skill_rpc.rs}}}`.
  - `docs/en/ENV_REFERENCE.md` + `docs/zh/ENV_REFERENCE.md` — appended
    "Advanced / Internal Tunables" section listing the newly-documented
    variables.
- Commits/changes: pure refactor + new test, no behaviour or default
  changes.

## Findings

- Critical: none.
- Major: none.
- Minor:
  - The two `starts_with("SKILLLITE_EVO…")` / `starts_with("SKILLLITE_EVOLUTION_")`
    prefix checks in `evolution_ui::is_evolution_runtime_env_key` are
    intentionally annotated with `// SKIP-ENV-AUDIT` (they are prefixes,
    not single env-var names). This is the only opt-out.
  - Pre-existing `private_interfaces` warning at
    `crates/skilllite-assistant/src-tauri/src/gateway_manager.rs:27` is
    unaffected by this PR (the assistant manifest is not in the workspace
    and the project CI runs `cargo clippy --all-targets -- -D warnings`
    on the workspace only). Out of scope.
  - Test fixtures inside `#[cfg(test)]` blocks
    (`crates/skilllite-sandbox/src/common.rs:701, 715`) intentionally use
    `SKILLLITE_UT_…` literals to exercise the env-compat helper. The
    consistency test correctly skips them via `production_prefix`.

## Quality Gates

- Architecture boundary checks: `pass`
  (`cargo deny check bans` → `bans ok`; no new cross-crate deps; the
  `skilllite-fs` mirror is documented to avoid an
  `skilllite-fs → skilllite-core` reverse dep that would otherwise fail
  `cargo deny`).
- Security invariants: `pass` (no env name renamed; alias chains
  preserved; `loader::DEPRECATED_PAIRS` table now references constants,
  same pairs).
- Required tests executed: `pass` (`cargo test --workspace` green; six
  new env_keys tests added; falsifiability hand-check confirmed both
  registry-coverage tests fail when a constant is removed).
- Docs sync (EN/ZH): `pass` (`docs/{en,zh}/ENV_REFERENCE.md` extended
  with the newly-registered variables; verbiage parallel between EN/ZH).

## Test Evidence

- Commands run:
  - `cargo fmt --check` (workspace) → clean.
  - `cargo fmt --check --manifest-path crates/skilllite-assistant/src-tauri/Cargo.toml`
    → clean.
  - `cargo clippy --all-targets -- -D warnings` (workspace) → clean.
  - `cargo build --workspace --all-targets` → clean.
  - `cargo build --manifest-path crates/skilllite-assistant/src-tauri/Cargo.toml`
    → clean (one pre-existing warning, not introduced here).
  - `cargo test --workspace` → all green.
  - `cargo test -p skilllite-core --lib config::env_keys` → 6 / 6 pass.
  - `cargo deny check bans` → `bans ok`.
  - `python3 scripts/validate_tasks.py` →
    `Task validation passed (64 task directories checked).`
  - Manual greps:
    - `rg 'env::var\("SKILLLITE_'  crates/*/src crates/skilllite-assistant/src-tauri/src skilllite/src`
      → no matches outside `env_keys.rs`.
    - `rg '(env_optional|env_or|env_bool|env_compat)\("SKILLLITE_'`
      → no matches outside `env_keys.rs`.
- Key outputs:
  - Falsifiability: removing
    `"SKILLLITE_FUZZY_THRESHOLD"` from `all_known_keys()` produced
    `Found 1 SKILLLITE_* literal(s) in production code that are not
    registered in env_keys::all_known_keys(): … skilllite-fs/src/search_replace.rs
    (line 331): SKILLLITE_FUZZY_THRESHOLD` and
    `all_known_keys() missing SKILLLITE_FUZZY_THRESHOLD`. Entry restored.

## Decision

- Merge readiness: `ready`
- Follow-up actions:
  - Future evolution: when adding any new `SKILLLITE_*` env var,
    register it in `env_keys.rs` AND add it to `all_known_keys()`. The
    consistency test will block PRs that forget either step.
  - Optional follow-up: rename
    `crates/skilllite-sandbox/src/common.rs` test fixture constants
    (`SKILLLITE_UT_ENV_COMPAT_NEW`, `SKILLLITE_UT_ENV_COMPAT_SET_A`)
    to a non-`SKILLLITE_` prefix for hygiene, since they are
    test-only and intentionally not in the registry. Not blocking.
