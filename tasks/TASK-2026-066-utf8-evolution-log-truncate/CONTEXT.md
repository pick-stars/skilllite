# Technical Context

## Current State

- Relevant crates/files: `crates/skilllite-commands/src/evolution_desktop.rs`, `crates/skilllite-commands/src/evolution.rs`.
- Current behavior: `log_manual_evolution_trigger()` copies the summary into a `String`, calls `truncate(480)`, and appends `…` when the byte length is above 480.

## Architecture Fit

- Layer boundaries involved: CLI command layer only; no dependency direction changes.
- Interfaces to preserve: `skilllite evolution run --log-manual-trigger`, evolution log event fields, and desktop L2 JSON contract.

## Dependency and Compatibility

- New dependencies: none.
- Backward compatibility notes: the logged reason may be a few bytes shorter when the limit lands inside a multi-byte character, but remains valid UTF-8 and keeps the same ellipsis marker.

## Design Decisions

- Decision: reuse the existing `truncate_utf8()` helper for manual trigger summaries.
  - Rationale: the helper already enforces character boundaries for pending skill previews in the same module.
  - Alternatives considered: count characters instead of bytes.
  - Why rejected: byte-budget behavior was already established and only needs boundary safety.

## Open Questions

- [x] Is a schema or documentation update required? No; behavior remains the same except crash prevention.
