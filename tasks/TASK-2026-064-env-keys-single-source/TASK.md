# TASK Card

## Metadata

- Task ID: `TASK-2026-064`
- Title: Consolidate SKILLLITE_* env-var literals into env_keys with consistency test
- Status: `done`
- Priority: `P1`
- Owner: `agent`
- Contributors:
- Created: `2026-04-29`
- Target milestone: post-`TASK-2026-063`

## Problem

`crates/skilllite-core/src/config/env_keys.rs` is documented as the canonical
registry of all `SKILLLITE_*` environment variables. However an audit shows
~40 production call sites still bypass it via `std::env::var("SKILLLITE_FOO")`,
`loader::env_optional("SKILLLITE_FOO", &[])`, or `cmd.env("SKILLLITE_FOO", …)`
string literals. This violates the single-source-of-truth intent and lets
mistyped names ship silently, contradicting `spec/architecture-boundaries.md`'s
"central config" rule and `spec/rust-conventions.md`'s "no magic strings for
external surface" guidance.

## Scope

- In scope:
  - Add canonical `pub const` entries to `env_keys.rs` for every `SKILLLITE_*`
    name found in workspace production code.
  - Replace bypass call sites in `skilllite-agent`, `skilllite-evolution`,
    `skilllite-sandbox`, `skilllite-commands`, `skilllite-executor`,
    `skilllite-swarm`, `skilllite-artifact`, `skilllite-assistant` with the
    constants.
  - Mirror constant for `skilllite-fs` (cannot depend on `skilllite-core` due
    to existing layering: `core` → `fs`). Document the mirror.
  - Add `env_keys::all_known_keys()` and a consistency test that scans
    `crates/*/src/**/*.rs` for `SKILLLITE_*` string literals and asserts each is
    in the registry. Test must be falsifiable (removing a constant should fail
    the test).
- Out of scope:
  - Renaming or deprecating any env var.
  - Changing alias-resolution logic (`loader::env_optional` semantics).
  - Behavioural changes (defaults, parsing) — this is purely a literal-to-const
    refactor.
  - Python SDK envs (separate sub-process).

## Acceptance Criteria

- [x] `env_keys.rs` lists every `SKILLLITE_*` name read or written in
      workspace production code (verified by the new consistency test).
- [x] No production `.rs` file outside `env_keys.rs` and the documented
      `skilllite-fs` mirror contains an `env::var("SKILLLITE_…")`,
      `env_optional("SKILLLITE_…")`, `set_var("SKILLLITE_…")`, or
      `cmd.env("SKILLLITE_…")` raw string literal.
- [x] `env_keys::all_known_keys()` returns a deduplicated, sorted list and is
      reachable from a public path.
- [x] New consistency test in `skilllite-core` walks `crates/*/src` and fails
      if any `SKILLLITE_*` literal is not registered. Falsifiable: deleting a
      constant entry breaks the test (verified by hand-perturbation in dry-run).
- [x] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
      `cargo test --workspace`, `cargo deny check bans` all pass.
- [x] `python3 scripts/validate_tasks.py` passes.

## Risks

- Risk: The consistency test produces false positives on tooling/build files
  that legitimately reference SKILLLITE names as documentation.
  - Impact: CI breaks on unrelated changes.
  - Mitigation: Test only scans `crates/*/src/**/*.rs` (Rust source) and
    excludes `env_keys.rs` itself. Doc strings in non-Rust files are ignored.
- Risk: Regex-based scanning misses an indirection (e.g. format!-built names).
  - Impact: A bypass slips through.
  - Mitigation: Audit pre-pass already enumerated every `SKILLLITE_*` substring
    in the workspace; no production call site uses `format!` or runtime
    concatenation for these names.
- Risk: `skilllite-fs` mirror desyncs from `env_keys.rs`.
  - Impact: Drift between canonical name and the actually-read literal.
  - Mitigation: Consistency test scans `skilllite-fs` too; the literal still
    must equal the registered constant string, so any drift fails CI.

## Validation Plan

- Required tests:
  - New: `skilllite_core::config::env_keys::tests::all_skilllite_env_literals_are_registered`
  - Existing crate-level tests for affected modules must keep passing.
- Commands to run:
  - `cargo fmt --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test -p skilllite-core --lib`
  - `cargo test --workspace`
  - `cargo deny check bans`
  - `python3 scripts/validate_tasks.py`
- Manual checks:
  - Run `rg -n 'env::var\("SKILLLITE_' crates/*/src` and confirm only
    `skilllite-fs` mirror remains as a known exception (or zero hits).
  - Hand-perturbation: temporarily delete one `pub const` line; confirm test
    fails; restore.

## Regression Scope

- Areas likely affected:
  - All crates listed in scope (mechanical literal-to-const swap).
  - Behaviour preserved: same env name strings, same default fallbacks, same
    alias chains.
- Explicit non-goals:
  - No env-var renames.
  - No new env vars introduced.
  - No CLI flag changes.

## Links

- Source TODO section: 全面项目分析报告 P1 #2
- Related PRs/issues:
- Related docs:
  - `crates/skilllite-core/src/config/env_keys.rs`
  - `crates/skilllite-core/src/config/loader.rs`
  - `spec/architecture-boundaries.md`
  - `spec/rust-conventions.md`
