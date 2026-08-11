# exec-convenience-api-001-C002 — NULL callback

Feature: `exec-convenience-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/legacy.c:30` (NULL callback = statements executed for side effects only)

## Boundary invoke
1. `rc = sqlite3_exec(db, "SELECT 1", NULL, NULL, NULL)`

## Observables to capture
- `exec.rc` only (no callback, no crash — the run itself is the observable)

## Scrub
- None.
