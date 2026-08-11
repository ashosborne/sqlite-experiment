# prepare-statement-api-005-C001 — reset preserves bindings

Feature: `prepare-statement-api-005` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/vdbeapi.c:134` (reset), card ("reset preserves bindings"; clear_bindings is the eraser)

## Preconditions / fixtures
- Prepared `"SELECT ?"`; bind_int(1, 42); stepped to ROW then DONE.

## Boundary invoke
1. `rc1 = sqlite3_reset(stmt)`
2. `rc2 = sqlite3_step(stmt)`  /* NO re-bind */
3. `v = sqlite3_column_int(stmt, 0)` /* bound value still present as implemented */

## Observables to capture
- `reset.rc`, `post_reset_step.rc`, `column_after_reset.value`

## Scrub
- None.
