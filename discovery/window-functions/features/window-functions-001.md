# window-functions-001 — Built-in window function family

Slice: `window-functions` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

11 built-in window functions registered in sqlite3WindowFunctions.

## Entrypoints (citations)

- `sqlite3WindowFunctions()` (other) — `src/window.c:610`, `src/window.c:147`

## Inputs / outputs / observables

- Results of row_number/rank/dense_rank/percent_rank/cume_dist/ntile/lag/lead/first_value/last_value/nth_value over OVER clauses

## Behaviour (as implemented)

- Registered by sqlite3WindowFunctions (src/window.c:610) with dedicated step/value implementations (e.g. row_numberStepFunc src/window.c:147); rank family requires ORDER BY in the window; lag/lead take offset+default args

## Validation rules found in code

- Window functions outside OVER context → error 'misuse of window function'

## Edge cases found in code

- ntile distributes remainder to leading groups; nth_value counts within frame not partition

## Dependencies

- select-codegen

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/window.c:610`
- `src/window.c:147`
