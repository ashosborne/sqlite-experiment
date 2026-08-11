# exec-convenience-api-001-C001 — exec with recording callback

Feature: `exec-convenience-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/legacy.c:30`

## Preconditions / fixtures
- Pinned baseline; fresh `:memory:` connection.

## Boundary invoke
1. `rc = sqlite3_exec(db, "SELECT 1", cb, &state, &zErr)` where `cb` increments `calls`, records
   `argc` and `argv[0]` (text), and returns 0.
2. `sqlite3_free(zErr)` regardless.

## Observables to capture
- `exec.rc`, `cb.calls`, `cb.argc`, `cb.argv0` (text form of the single column)

## Scrub
- None (static input).
