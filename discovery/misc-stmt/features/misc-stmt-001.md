# misc-stmt-001 — sqlite_stmt introspection vtab

Slice: `misc-stmt` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Rows describe currently-prepared statements (sql, busy, counters)

## Entrypoints (citations)

- `sqlite3_stmt_init()` (other) — `ext/misc/stmt.c:334`

## Inputs / outputs / observables

- SELECT * FROM sqlite_stmt — one row per prepared statement (sql, ncol, ro, busy, nscan, nsort, naidx, nstep, reprep, run, mem)

## Behaviour (as implemented)

- init ext/misc/stmt.c:334: eponymous vtab iterating sqlite3_next_stmt with per-statement status counters

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- The SELECT reading sqlite_stmt sees itself in the list

## Dependencies

- vtab-core
- prepare-statement-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/stmt.c:334`
