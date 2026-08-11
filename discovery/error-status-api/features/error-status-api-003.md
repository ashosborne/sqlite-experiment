# error-status-api-003 — Status counters (global + per-connection)

Slice: `error-status-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

status64/db_status expose current/highwater counters with optional reset.

## Entrypoints (citations)

- `sqlite3_status64()` (api) — `src/status.c:134`, `src/status.c:159`, `src/status.c:426`

## Inputs / outputs / observables

- (current, highwater) pairs per op; resetFlag zeroes highwater; db_status ops incl. LOOKASIDE_USED, CACHE_USED, SCHEMA_USED, STMT_USED

## Behaviour (as implemented)

- sqlite3_status64/status (src/status.c:134,159) read process-global counters (MEMORY_USED, PAGECACHE_*, MALLOC_*) under the pcache mutex
- sqlite3_db_status (src/status.c:426) computes per-connection stats, some by walking schema/statement structures on demand

## Validation rules found in code

- Unknown op → SQLITE_MISUSE

## Edge cases found in code

- Some db_status ops (CACHE_HIT/MISS) are per-pager aggregates reset only via resetFlag

## Dependencies

- malloc-subsystem

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/status.c:134`
- `src/status.c:159`
- `src/status.c:426`
