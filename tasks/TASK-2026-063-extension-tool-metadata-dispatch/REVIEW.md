# Review Report

## Scope Reviewed

- Files/modules:
  - `crates/skilllite-agent/src/extensions/registry.rs` — added `PlanningControlKind` enum, `ResultProcessingProfile` enum (Default = `Standard`), `RegisteredTool::result_processing` field + `with_result_processing_profile` builder, `ExtensionRegistry::result_processing_profile` and `ExtensionRegistry::planning_control_kind` lookups, `ToolHandler::PlanningControl(PlanningControlKind)` (was unit variant), `PlanningControlExecutor::execute(kind: PlanningControlKind, ...)` trait change, two new unit tests for the lookups.
  - `crates/skilllite-agent/src/extensions/mod.rs` — re-export `PlanningControlKind` and `ResultProcessingProfile`.
  - `crates/skilllite-agent/src/extensions/builtin/mod.rs` — added `planning_control_kind_for` and `result_processing_profile_for` helpers; `get_builtin_tools()` now sets the registration metadata that previously lived as `tool_name == "..."` checks in `agent_loop/`.
  - `crates/skilllite-agent/src/agent_loop/execution.rs` — replaced `tool_name == "update_task_plan"` / `tool_name == "complete_task"` (4 sites) with `registry.planning_control_kind(tool_name)` and `matches!(planning_kind, Some(PlanningControlKind::CompleteTask))`. `PlanningControlExecutorImpl` now `match`es on `PlanningControlKind`. Both `process_result_content` callers pass `result_profile` retrieved from the registry instead of `tool_name`.
  - `crates/skilllite-agent/src/agent_loop/helpers.rs` — `process_result_content` now takes `ResultProcessingProfile`; the `CONTENT_PRESERVING_TOOLS` const and the `tool_name == "read_file"` early-return are gone.
- Commits/changes: Single PR; +156 / −74 LOC net (refactor + new tests + new metadata table).

## Findings

- Critical: None.
- Major: None.
- Minor:
  - `RegisteredTool::accepts_legacy_field_shape` still hardcodes `complete_task` / `update_task_plan` field-shape compatibility in `extensions/registry.rs` (lines 528–536). This is in the **extension layer** (allowed by `spec/architecture-boundaries.md` — the spec only forbids such checks in `agent_loop/`), and lifting it would touch the legacy JSON-shape compatibility path documented as paired-test territory in `spec/verification-integrity.md` known-failure-pattern #3. Out of scope for this task; can be a follow-up if/when the planning-control surface grows.
  - The new `ResultProcessingProfile` enum has only three variants today; future content-preserving tools with intermediate caps would need either a fourth variant or a richer profile struct. Acceptable: enum is `#[non_exhaustive]`-compatible because adding a variant is a deliberate change and exhaustive `match` will fail to compile on the unhandled arm.

## Quality Gates

- Architecture boundary checks: `pass` — `rg -n 'tool_name\s*==\s*"|tool_name\.as_str\(\)\s*==' crates/skilllite-agent/src/agent_loop/` returns zero matches; `cargo deny check bans` ok in both root and assistant manifests.
- Security invariants: `pass` — no change to sandbox / capability policy / permission gates.
- Required tests executed: `pass` — `cargo test -p skilllite-agent` (243 passed, +2 from new tests) and `cargo test --workspace` (all suites pass).
- Docs sync (EN/ZH): `N/A` — internal refactor; no user-visible CLI / env / behavior change. `spec/docs-sync.md` requires sync only when behavior, commands, env vars, architecture docs, or release matrix change.

## Test Evidence

- Commands run:
  - `cargo fmt --check` → exit 0 (after running `cargo fmt`).
  - `cargo clippy --workspace --all-targets -- -D warnings` → exit 0.
  - `cargo test -p skilllite-agent` → 243 passed; 0 failed; 0 ignored.
  - `cargo test --workspace` → all suites pass (counts unchanged for non-agent crates; +2 in skilllite-agent).
  - `cargo deny check bans` → `bans ok` (root workspace).
  - `cargo deny --manifest-path crates/skilllite-assistant/src-tauri/Cargo.toml check bans` → `bans ok` (assistant manifest).
  - `python3 scripts/validate_tasks.py` → "Task validation passed (63 task directories checked)".
  - `rg -n 'tool_name\s*==\s*"|tool_name\.as_str\(\)\s*==' crates/skilllite-agent/src/agent_loop/` → zero matches.
- Key outputs:
  - New tests `planning_control_kind_lookup_returns_typed_enum_for_registered_tools` and `result_processing_profile_lookup_falls_back_to_standard_for_unknown_tools` both pass.
  - Existing planning / execution / chat_history / read_file tests in `extensions/builtin/tests.rs` and `agent_loop/execution.rs` (`test_complete_task_with_string_task_id_succeeds`, `test_complete_task_rejects_success_when_failures_or_replans_exist`, `test_complete_task_auto_recovery_after_repeated_failures`, `test_update_task_plan_with_stringified_tasks_array_succeeds`, etc.) all still pass — proving the typed-enum dispatch preserves the prior runtime behavior of the planning-control auto-recover, replan-soft-limit, and completion-conflict paths.

## Decision

- Merge readiness: `ready`
- Follow-up actions:
  - Optional: lift `RegisteredTool::accepts_legacy_field_shape` to a metadata bit too (low value, deferred; flagged as Minor finding).
  - The 2026-04-29 architecture review (parent §1.2 / §1.3) lists two more P1 items (env_keys consolidation; Python SDK `ipc.py` silent except). Those are independent tasks.
