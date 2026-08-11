# connection-lifecycle-api-004 — Connection hooks and tracing

Slice: `connection-lifecycle-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

commit_hook/update_hook/trace_v2 register observable per-connection callbacks.

## Entrypoints (citations)

- `sqlite3_trace_v2()` (api) — `src/main.c:2309`, `src/main.c:2369`, `src/main.c:2394`

## Inputs / outputs / observables

- trace_v2 callback events SQLITE_TRACE_STMT/PROFILE/ROW/CLOSE with expanded-SQL availability
- commit_hook return non-zero converts COMMIT into ROLLBACK (constraint-style)
- update_hook fires (op, dbname, table, rowid) per row change on rowid tables

## Behaviour (as implemented)

- sqlite3_trace_v2 (src/main.c:2309) registers mask-filtered callback replacing legacy trace/profile
- sqlite3_commit_hook (src/main.c:2369) callback runs before commit finalises; non-zero return aborts the commit
- sqlite3_update_hook (src/main.c:2394) fires for INSERT/UPDATE/DELETE on rowid tables only

## Validation rules found in code

- Registering returns the previous callback's user-arg (replacement semantics)

## Edge cases found in code

- update_hook does not fire for WITHOUT ROWID tables or for changes via ON CONFLICT REPLACE internal deletes in some paths (truncate optimization disables hooks-compatible fast path)
- ROW trace fires per result row, not per step retry

## Dependencies

- vdbe-engine

## Assumptions / unknowns

- Legacy sqlite3_trace/sqlite3_profile still present but deprecated — run-1 SME question stands
- Are legacy sqlite3_trace/profile in scope or deprecated-out?

## Evidence

- `src/main.c:2309`
- `src/main.c:2369`
- `src/main.c:2394`
