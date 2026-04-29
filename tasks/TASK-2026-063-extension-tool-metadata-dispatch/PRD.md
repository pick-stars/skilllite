# PRD

## Background

The agent loop is the central orchestrator and must remain extension-agnostic. Today three string compares (`tool_name == "..."`) leak tool-specific knowledge from `extensions/` into `agent_loop/`, contradicting `spec/architecture-boundaries.md`. Each new planning-control or content-preserving tool would therefore require editing `agent_loop`, which is exactly what the spec forbids. The 2026-04-29 architecture review (parent §1.1) flagged this as the highest-priority code fix.

## Objective

Replace the three `tool_name == "..."` branches under `crates/skilllite-agent/src/agent_loop/` with typed-enum dispatch driven by metadata that lives on `RegisteredTool` in `extensions/registry.rs`. After this change, adding a new planning-control or content-preserving tool requires editing only `extensions/`, never `agent_loop/`.

## Functional Requirements

- FR-1: `RegisteredTool` exposes `result_processing_profile() -> ResultProcessingProfile` and the `ExtensionRegistry` provides a public lookup `result_processing_profile(tool_name)` returning `Standard` for unknown tools.
- FR-2: `ToolHandler::PlanningControl` carries a typed `PlanningControlKind { UpdateTaskPlan, CompleteTask }`; `PlanningControlExecutor::execute` receives `PlanningControlKind` instead of a raw `&str`.
- FR-3: `agent_loop/execution.rs` and `agent_loop/helpers.rs` contain no `tool_name == "<literal>"` branches.
- FR-4: Existing built-in tools (`read_file`, `chat_history`, `complete_task`, `update_task_plan`) preserve their current runtime behavior — the metadata table in `extensions/builtin/mod.rs` reproduces the prior dispatch results bit-for-bit.

## Non-Functional Requirements

- Security: No change to sandbox/permission/policy paths. Trait remains internal to `skilllite-agent`.
- Performance: Lookup is `HashMap::get`; equivalent to the existing string compare. No measurable impact.
- Compatibility: Internal API change. `PlanningControlExecutor` is a crate-internal trait re-exported through `extensions::`; only the in-tree `PlanningControlExecutorImpl` implements it.

## Constraints

- Technical: Rust workspace, must satisfy `spec/rust-conventions.md` (no `unwrap`/`expect` in production paths, no `anyhow::*` in crates, zero clippy warnings).
- Timeline: Single-session refactor; no migration window required.

## Success Metrics

- Metric 1: count of `tool_name == "<literal>"` branches under `crates/skilllite-agent/src/agent_loop/`
  - Baseline: 3 (execution.rs:269, execution.rs:286, helpers.rs:372)
  - Target: 0
- Metric 2: number of new planning-control tool registrations that require editing `agent_loop`
  - Baseline: every new planning-control tool needs an `else if` branch
  - Target: 0 (must extend the `PlanningControlKind` enum and its `match`, both in `extensions/`)
- Metric 3: regressions in existing tests
  - Baseline: 0
  - Target: 0 (existing 241 `skilllite-agent` tests must all still pass)

## Rollout

- Rollout plan: Single PR. Internal refactor; no env / CLI / docs change.
- Rollback plan: `git revert` the PR. The change is mechanical and local to `skilllite-agent`; no data migration.
