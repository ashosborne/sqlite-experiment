# window-functions-002 — Frame specification execution (ROWS/RANGE/GROUPS + EXCLUDE)

Slice: `window-functions` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Frame boundary codegen and per-row step/inverse calls.

## Entrypoints (citations)

- `sqlite3WindowCodeStep()` (other) — `src/window.c:2784`, `src/window.c:659`

## Inputs / outputs / observables

- Frame semantics: ROWS/RANGE/GROUPS with UNBOUNDED/CURRENT/offset PRECEDING/FOLLOWING bounds and EXCLUDE variants; aggregate-over-window results

## Behaviour (as implemented)

- sqlite3WindowCodeStep (src/window.c:2784) drives per-row frame maintenance calling step/inverse; sqlite3WindowUpdate (src/window.c:659) rewrites/validates the window spec and maps aggregates to window mode
- Default frame RANGE UNBOUNDED PRECEDING TO CURRENT ROW; RANGE with offsets requires numeric ORDER BY key

## Validation rules found in code

- Frame bound legality checked at compile (e.g. start after end → error)

## Edge cases found in code

- EXCLUDE CURRENT ROW/GROUP/TIES interact with peers under RANGE — behaviour-dense; inverse-function correctness is what makes big frames O(n)

## Dependencies

- vdbe-engine

## Assumptions / unknowns

- Aggregate-vs-window card ownership resolved: aggregates' inverse funcs live in builtin-scalar-agg-funcs-002; frame mechanics here
- Aggregate-as-window (sum OVER ...) shares inverse funcs — bind decision on where those cards live

## Evidence

- `src/window.c:2784`
- `src/window.c:659`
