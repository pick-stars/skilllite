# Technical Context

## Current State

- Relevant crates/files:
  - `crates/skilllite-agent/src/extensions/registry.rs` — `RegisteredTool`, `ToolHandler::PlanningControl`, `PlanningControlExecutor` trait, `execute()` dispatch.
  - `crates/skilllite-agent/src/extensions/builtin/mod.rs` — `get_builtin_tools()` registration; chooses `ToolHandler::PlanningControl` for `complete_task` / `update_task_plan` based on a `matches!()` on the tool name.
  - `crates/skilllite-agent/src/agent_loop/execution.rs` — `PlanningControlExecutorImpl::execute` matches on `tool_name == "update_task_plan"` / `"complete_task"`.
  - `crates/skilllite-agent/src/agent_loop/helpers.rs` — `process_result_content` matches on `tool_name == "read_file"` and uses a `CONTENT_PRESERVING_TOOLS` const string slice.
- Current behavior:
  - `read_file` uses `process_read_file_tool_result_content` (larger cap, `SKILLLITE_READ_FILE_TOOL_RESULT_MAX_CHARS`); never LLM-summarized.
  - `chat_history` is in `CONTENT_PRESERVING_TOOLS`: standard cap, head+tail truncation when oversized; never LLM-summarized.
  - All other tools use the standard `SKILLLITE_TOOL_RESULT_MAX_CHARS` cap and may go through LLM summarization on overflow.
  - `complete_task` and `update_task_plan` are dispatched through `PlanningControlExecutor` keyed by their string name.

## Architecture Fit

- Layer boundaries involved: `agent_loop` is the orchestrator; `extensions` owns tool definitions, capabilities, and execution profiles. Per `spec/architecture-boundaries.md` MUST NOT, `agent_loop` may not branch on tool names.
- Interfaces to preserve:
  - `RegisteredTool::new`, `with_scope`, `with_execution_profile` (additive change: a new `with_result_processing_profile`).
  - `ExtensionRegistry::execute` public signature (no change).
  - `ExtensionRegistry::tool_profile` (no change).
- Interfaces changing (internal trait):
  - `PlanningControlExecutor::execute(&mut self, kind: PlanningControlKind, arguments: &str, event_sink: &mut dyn EventSink) -> ToolResult` (was `tool_name: &str`).
  - `ToolHandler::PlanningControl` becomes `ToolHandler::PlanningControl(PlanningControlKind)` (a `Copy` enum).

## Dependency and Compatibility

- New dependencies: none.
- Backward compatibility notes:
  - `RegisteredTool::new` keeps its three-arg signature; the new profile defaults to `Standard`. Builders that did not set a profile keep current behavior. The two callers that do need a non-default value (`read_file`, `chat_history`) are updated in `extensions/builtin/mod.rs` registration.
  - `PlanningControlExecutor` trait is re-exported from `crate::extensions`. Only one in-tree impl exists (`PlanningControlExecutorImpl` in `agent_loop/execution.rs`); any out-of-tree implementor would need to migrate, but this is a crate-internal contract.

## Design Decisions

- Decision: Model result-processing as a closed `enum ResultProcessingProfile { Standard, ContentPreservingStandard, ContentPreservingLarge }` rather than two booleans.
  - Rationale: The three observable behaviors (`Standard` LLM-summarize-on-overflow, `ContentPreservingStandard` head+tail truncate at standard cap, `ContentPreservingLarge` head+tail truncate at the larger read_file cap) are mutually exclusive and not all 2×2 boolean combinations are meaningful.
  - Alternatives considered: `{ never_summarize: bool, use_large_cap: bool }`.
  - Why rejected: `{ never_summarize=false, use_large_cap=true }` has no defined semantics; the enum models the actual decision tree without producing junk states.

- Decision: Add `PlanningControlKind` as a typed enum in `extensions::registry`.
  - Rationale: Forces exhaustive `match` in the agent loop; adding a new variant fails to compile until the dispatcher is updated, which is the right ergonomics for a deliberate architectural change.
  - Alternatives considered: `Box<dyn PlanningControlOp>` per tool registered separately.
  - Why rejected: Premature abstraction; the planning-control surface has been stable at two tools for the entire history of TASK-2026-002 / TASK-2026-004 and growth requires careful design (anti-loop guards, completion-conflict detection) which already lives in the agent loop.

- Decision: Keep `validate_input`'s legacy `write_file` / `write_output` JSON-recovery branch and the planning legacy field-shape compatibility unchanged.
  - Rationale: Out of scope; those paths are documented compatibility layers (see `spec/verification-integrity.md` known-failure-pattern #3 about paired tests). Refactoring them risks regressions on JSON recovery + Gemini compatibility.
  - Alternatives considered: Lift `accepts_legacy_field_shape` to a profile bit too.
  - Why rejected: Scope creep; can be a follow-up if needed.

## Open Questions

- [x] Should the trait `PlanningControlExecutor` move from `extensions::registry` to `agent_loop::` since only the agent loop implements it? — **Decision**: keep it in `extensions::registry` because the registry's `execute()` method calls into it; moving the trait would invert the dependency direction. Resolved.
- [x] Should `chat_history` keep its larger budget or share `read_file`'s cap? — **Decision**: preserve current behavior (`ContentPreservingStandard`), this task is a refactor, not a tuning change. Resolved.
