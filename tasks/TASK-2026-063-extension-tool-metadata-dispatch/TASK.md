# TASK Card

## Metadata

- Task ID: `TASK-2026-063`
- Title: Replace agent_loop tool_name string branches with RegisteredTool metadata
- Status: `done`
- Priority: `P1`
- Owner: `agent`
- Contributors:
- Created: `2026-04-29`
- Target milestone:

## Problem

`spec/architecture-boundaries.md` MUST NOT: *"Do not add new tools in `agent_loop` via `if tool_name == 'xxx'` branching"*. Three such branches still exist in the agent crate:

- `crates/skilllite-agent/src/agent_loop/execution.rs:269` — `if tool_name == "update_task_plan"` in `PlanningControlExecutorImpl::execute`.
- `crates/skilllite-agent/src/agent_loop/execution.rs:286` — `else if tool_name == "complete_task"` (same dispatcher).
- `crates/skilllite-agent/src/agent_loop/helpers.rs:372` — `if tool_name == "read_file"` in `process_result_content` to choose a content-preserving truncation budget.

Symptom: every new planning-control tool or content-preserving tool requires editing the orchestration layer (`agent_loop`) instead of registering self-describing metadata in `extensions/`.

## Scope

- In scope:
  - Add structured metadata to `RegisteredTool` so `agent_loop` dispatches by typed enum, not by string match.
  - Lift the `read_file` / `chat_history` content-preserving behavior to a `ResultProcessingProfile` enum on `RegisteredTool` looked up via `ExtensionRegistry`.
  - Lift planning-control routing to a `PlanningControlKind { UpdateTaskPlan, CompleteTask }` enum carried by `ToolHandler::PlanningControl(kind)`.
  - Update `PlanningControlExecutor` trait to receive the typed `PlanningControlKind` instead of `&str`.
  - Update built-in tool registration (`extensions/builtin/mod.rs`) to attach the new metadata for the existing tools.
  - Tests covering: registry → profile lookup, planning-control routing through enum, default profile behavior for unregistered tools, and existing happy-path tests still pass.
- Out of scope:
  - Refactor of MCP tool dispatch.
  - Splitting `chat_session.rs` (separate task).
  - Any change to the `extensions/builtin/mod.rs` `match tool_name` table — that one already lives in the extension layer (allowed by the spec).
  - Behavioral change to truncation thresholds, summarization model, or env keys.

## Acceptance Criteria

- [x] No `tool_name == "update_task_plan"`, `tool_name == "complete_task"`, or `tool_name == "read_file"` branches remain under `crates/skilllite-agent/src/agent_loop/`.
- [x] `PlanningControlExecutor::execute` signature takes `PlanningControlKind` (not `&str`).
- [x] `RegisteredTool` carries `result_processing_profile()` and the registry exposes a lookup; default value is `Standard`.
- [x] All existing built-in tools register the metadata that preserves prior runtime behavior (read_file → ContentPreservingLarge, chat_history → ContentPreservingStandard, complete_task → PlanningControlKind::CompleteTask, update_task_plan → PlanningControlKind::UpdateTaskPlan).
- [x] `cargo fmt --check` clean.
- [x] `cargo clippy --workspace --all-targets` zero warnings.
- [x] `cargo test --workspace` passes (no test was deleted to silence failures; new tests were added).
- [x] `cargo deny check bans` ok (root + assistant manifest).
- [x] `python3 scripts/validate_tasks.py` passes.

## Risks

- Risk: Behavioral drift in result truncation if `ResultProcessingProfile` mapping is wrong for `read_file` or `chat_history`.
  - Impact: LLM could lose access to large file content (regression on long-content tools).
  - Mitigation: Mirror the existing `process_read_file_tool_result_content` and `CONTENT_PRESERVING_TOOLS` semantics; add unit test asserting the lookup for those two tools.
- Risk: `PlanningControlKind` enum becomes a closed set; future planning tools require enum extension.
  - Impact: Slightly higher friction for adding planning-control tools (still strictly less than current state).
  - Mitigation: Acceptable — adding a planning-control tool is a deliberate architectural change, not a casual extension; the trait now forces enum exhaustiveness so the agent loop fails to compile if a new variant is unhandled.
- Risk: Trait signature change is a breaking API change for any out-of-tree consumer of `PlanningControlExecutor`.
  - Impact: External implementors must update their `execute` method.
  - Mitigation: Trait is internal to `skilllite-agent` (only one impl in-tree); no public re-export of the trait outside the crate's `extensions::` module surface that downstream relies on.

## Validation Plan

- Required tests (per `spec/testing-policy.md` for skilllite-agent changes):
  - `cargo test -p skilllite-agent`
  - One regression-focused unit test for new metadata lookup (happy + unknown-tool fallback path).
- Commands to run:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets`
  - `cargo test --workspace`
  - `cargo deny check bans`
  - `cargo deny --manifest-path crates/skilllite-assistant/src-tauri/Cargo.toml check bans`
  - `python3 scripts/validate_tasks.py`
- Manual checks:
  - Re-grep for `tool_name == "` under `agent_loop/` to confirm zero residue.

## Regression Scope

- Areas likely affected:
  - `skilllite-agent::extensions::registry` (trait + RegisteredTool API).
  - `skilllite-agent::extensions::builtin` (registration metadata).
  - `skilllite-agent::agent_loop::{execution, helpers}` (dispatch sites).
- Explicit non-goals:
  - No change to user-visible truncation behavior.
  - No change to skill / MCP / memory tool dispatch paths.
  - No change to env variables, CLI surface, or docs (this is an internal refactor; behavior is preserved).

## Links

- Source TODO section: 2026-04-29 全面分析报告 §1.1 "agent_loop 中残留按 tool_name == ... 硬分支".
- Related PRs/issues: Builds on TASK-2026-004-tool-lifecycle-contract (introduced `ToolExecutionProfile` / `ToolCapability`).
- Related docs: `spec/architecture-boundaries.md`, `spec/structured-signal-first.md`.
