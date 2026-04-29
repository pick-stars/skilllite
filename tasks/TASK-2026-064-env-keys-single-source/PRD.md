# PRD

## Background

`crates/skilllite-core/src/config/env_keys.rs` was introduced as the single
source of truth for `SKILLLITE_*` environment variables and their alias
fallback chains. Despite this, an audit found ~40 production read sites and
several write sites still embedding the env-var name as a magic string. The
audit also surfaced previously-undocumented variables (e.g.
`SKILLLITE_TRANSCRIPT_FLUSH_*`, `SKILLLITE_GOAL_LLM_EXTRACT`,
`SKILLLITE_FUZZY_THRESHOLD`, etc.) that are silently part of the public
configuration surface.

This task closes the loop: every `SKILLLITE_*` name lives in `env_keys.rs`,
every code site reads/writes it via the constant, and a test enforces the
invariant in CI.

## Objective

After this task:

1. 100% of `SKILLLITE_*` literal occurrences in workspace production code
   either (a) resolve through a `env_keys` constant or (b) are themselves the
   `pub const` declaration in `env_keys.rs` / its documented `skilllite-fs`
   mirror.
2. A single function `env_keys::all_known_keys()` returns the canonical set,
   and a test guards that any new bypass fails CI.

## Functional Requirements

- FR-1: `env_keys::all_known_keys() -> &'static [&'static str]` exposes a
  deduplicated, lexicographically-sorted slice of every registered name.
- FR-2: Every previously-bypassed call site reads via the corresponding
  constant. Behaviour (default value, alias chain, parsing) is unchanged.
- FR-3: A new `#[test]` in `skilllite-core` walks the workspace Rust source
  under `crates/*/src/**/*.rs`, extracts every `SKILLLITE_[A-Z0-9_]+` literal,
  and asserts the set is a subset of `all_known_keys()`. The test ignores
  `env_keys.rs` itself (the registry) and excludes test-only files
  (paths matching `tests.rs`, `*_tests.rs`, or `/tests/`).
- FR-4: For `skilllite-fs` (which cannot depend on `skilllite-core` due to
  reverse layering), the env-var name is declared as a file-local
  `pub(crate) const` whose value matches the canonical
  `env_keys::fs::SKILLLITE_FUZZY_THRESHOLD`. The mirror is documented in
  `env_keys.rs`.

## Non-Functional Requirements

- Security: No change. (Same env names, same alias chains, same defaults.)
- Performance: Negligible — `pub const &str` references compile to identical
  string-table entries; no runtime cost.
- Compatibility: Backward-compatible. No renames; alias resolution untouched.

## Constraints

- Technical:
  - Cannot create `skilllite-fs → skilllite-core` dependency (would invert
    existing layering and break `cargo deny`).
  - Test must run in standard `cargo test --lib` for `skilllite-core`; no
    workspace-walk hack via `xtask` for now.
- Timeline: Single PR, mechanical refactor.

## Success Metrics

- Metric: Production-code occurrences of `env::var("SKILLLITE_…")` literal
  outside `env_keys.rs` and the `skilllite-fs` mirror.
- Baseline: ≥ 21 (Rust read sites) plus indirect bypasses via
  `env_optional("SKILLLITE_…")`.
- Target: 0 (with the `skilllite-fs` mirror as a single, documented exception
  whose literal value must equal the registered constant — also enforced by
  the test).

## Rollout

- Rollout plan: One PR; merge after CI green. No env-var rename, no behaviour
  change → no user-visible release notes required (still update
  `docs/en/REFERENCE/ENV_REFERENCE.md` to surface newly-documented vars per
  `spec/docs-sync.md`).
- Rollback plan: Pure refactor. Revert the PR; existing literals still work
  (we never changed any value).
