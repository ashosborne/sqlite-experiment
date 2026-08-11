# exec-convenience-api-002 — get_table / free_table result marshalling

Slice: `exec-convenience-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Materialises full result set into a char* array with header row; caller frees via sqlite3_free_table.

## Entrypoints (citations)

- `sqlite3_get_table()` (api) — `src/table.c:116`, `src/table.c:185`

## Inputs / outputs / observables

- Result table layout: (nRow+1)*nCol char* array with header row first; NULL cells as NULL pointers; pnRow/pnColumn out-params

## Behaviour (as implemented)

- sqlite3_get_table (src/table.c:116) materialises the full result via sqlite3_exec into a single heap structure; sqlite3_free_table (src/table.c:185) releases it
- Mixed-column-count results across statements → SQLITE_ERROR

## Validation rules found in code

- Result must be freed only via sqlite3_free_table (internal layout has a length prefix)

## Edge cases found in code

- Zero-row result still yields header row and pnRow=0

## Dependencies

- exec-convenience-api-001

## Assumptions / unknowns

- Legacy interface; run-1 defer recommendation stands — bound here by ACCEPT_ALL policy
- Deprecated in docs? Verify migration target need.

## Evidence

- `src/table.c:116`
- `src/table.c:185`
