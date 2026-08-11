# prepare-statement-api-003-C001 — bind_int then step to ROW

Feature: `prepare-statement-api-003` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/vdbeapi.c:1892`, `src/vdbeapi.c:1816` (bind family), card prepare-statement-api-003

## Preconditions / fixtures
- Fresh `:memory:` connection; prepared statement `"SELECT ?"`.

## Boundary invoke
1. `rc1 = sqlite3_bind_int(stmt, 1, 7)`
2. `rc2 = sqlite3_step(stmt)` (shape: ROW)
3. `v = sqlite3_column_int(stmt, 0)`

## Observables to capture
- `bind.rc`, `step.rc`, `column_int.value`

## Scrub
- None.
