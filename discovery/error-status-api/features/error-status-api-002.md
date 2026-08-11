# error-status-api-002 — Runtime limits (sqlite3_limit)

Slice: `error-status-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Get/set 12 limit categories with hard upper bounds from sqliteLimit.h.

## Entrypoints (citations)

- `sqlite3_limit()` (api) — `src/main.c:3049`, `src/sqliteLimit.h`

## Inputs / outputs / observables

- sqlite3_limit returns the PRIOR value; effects visible via constraint errors (SQLITE_TOOBIG etc.)

## Behaviour (as implemented)

- sqlite3_limit (src/main.c:3049) gets/sets 12 per-connection limits (LENGTH, SQL_LENGTH, COLUMN, EXPR_DEPTH, COMPOUND_SELECT, VDBE_OP, FUNCTION_ARG, ATTACHED, LIKE_PATTERN_LENGTH, VARIABLE_NUMBER, TRIGGER_DEPTH, WORKER_THREADS); newVal<0 = query only
- Values clamped to compile-time hard maxima from src/sqliteLimit.h

## Validation rules found in code

- Invalid limit id → -1

## Edge cases found in code

- Lowering a limit does not invalidate existing prepared statements that already exceed it

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/main.c:3049`
- `src/sqliteLimit.h`
