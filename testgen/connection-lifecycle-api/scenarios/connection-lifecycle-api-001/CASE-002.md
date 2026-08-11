# connection-lifecycle-api-001-C002 — close that handle

Feature: `connection-lifecycle-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/main.c:1381` (sqlite3_close)

## Preconditions / fixtures
- Continuation of C001: the same live handle, no statements or backups outstanding.

## Boundary invoke
1. `rc = sqlite3_close(db)`

## Observables to capture
- `close.rc`

## Scrub
- None.
