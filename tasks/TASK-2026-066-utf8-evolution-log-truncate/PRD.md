# PRD

## Background

The desktop split-ready evolution bridge now records a manual trigger event after `skilllite evolution run --log-manual-trigger`. The event reason is derived from the user-visible evolution response and can contain CJK or emoji text. Rust `String::truncate(n)` requires `n` to be a valid UTF-8 character boundary; using a raw byte limit can panic.

## Objective

Manual evolution trigger logging must never crash because of non-ASCII summary text. The fix should preserve the existing audit event shape and only alter the clipping implementation.

## Functional Requirements

- FR-1: Clip manual evolution trigger summaries to the existing 480-byte budget without splitting UTF-8 characters.
- FR-2: Preserve the trailing ellipsis behavior when clipping occurs.

## Non-Functional Requirements

- Security: no security policy relaxation.
- Performance: clipping remains linear in the summary length and only runs once per manual trigger.
- Compatibility: log event type, target ID, workspace field, and JSON/CLI outputs remain unchanged.

## Constraints

- Technical: use existing local helper where possible; no new dependencies.
- Timeline: autonomous critical bug fix; no calendar estimate.

## Success Metrics

- Metric: regression test with repeated CJK text passes without panic.
- Baseline: `String::truncate(480)` panics when 480 is not a character boundary.
- Target: UTF-8-safe clipping returns valid text and appends `…`.

## Rollout

- Rollout plan: ship as a minimal Rust patch in `skilllite-commands`.
- Rollback plan: revert the single helper usage and test if unexpected behavior appears.
