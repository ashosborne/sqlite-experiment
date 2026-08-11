# connection-lifecycle-api-002 — Close database (deferred close semantics)

Slice: `connection-lifecycle-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_close fails with SQLITE_BUSY on unfinalized statements; close_v2 defers (zombie connection).

## Entrypoints (citations)

- `sqlite3_close_v2()` (api) — `src/main.c:1381`, `src/main.c:1382`, `src/sqlite.h.in:356`

## Inputs / outputs / observables

- sqlite3_close → SQLITE_BUSY (with error message 'unable to close due to unfinalized statements or unfinished backups') vs SQLITE_OK
- sqlite3_close_v2 → always SQLITE_OK (deferred/zombie close)

## Behaviour (as implemented)

- sqlite3Close(db,0) refuses to close while prepared statements or backups remain (src/main.c:1381)
- sqlite3Close(db,1) (close_v2) marks the connection a zombie; teardown happens when the last statement/backup finishes (src/main.c:1382)

## Validation rules found in code

- Both return SQLITE_MISUSE on invalid/already-closed handle (guarded by safety checks)

## Edge cases found in code

- Close inside an active transaction rolls the transaction back
- NULL pointer argument is a harmless no-op returning SQLITE_OK (documented behaviour)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/main.c:1381`
- `src/main.c:1382`
- `src/sqlite.h.in:356`
