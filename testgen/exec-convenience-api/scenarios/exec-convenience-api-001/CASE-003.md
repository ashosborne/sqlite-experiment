# exec-convenience-api-001-C003 — callback aborts

Feature: `exec-convenience-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/legacy.c:30` + card ("Non-zero callback return aborts with SQLITE_ABORT")

## Boundary invoke
1. `rc = sqlite3_exec(db, "SELECT 1", cb_abort, &state, &zErr)` where `cb_abort` increments `calls`
   and returns 1 on the first row.
2. `sqlite3_free(zErr)`.

## Observables to capture
- `exec.rc` (card shape: abort — recorded, not asserted), `cb.calls`

## Scrub
- None.
