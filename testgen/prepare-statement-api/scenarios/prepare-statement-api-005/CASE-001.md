# prepare-statement-api-005-C001 — reset preserves bindings

Feature: `prepare-statement-api-005` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/vdbeapi.c:134` (reset), card ("reset preserves bindings"; clear_bindings is the eraser)

## Preconditions / fixtures
- Prepared `"SELECT ?"`; bind_int(1, 42); stepped to ROW then DONE.

## Note on the autoreset pin
- This is **reset-from-DONE**: the harness deliberately steps to DONE first, then calls sqlite3_reset explicitly. On this pin (OMIT_AUTORESET=off) a bare re-step would auto-reset (that path is already pinned as 002-C002) — this case pins the EXPLICIT reset contract + binding preservation, which is a different observable set.

## Boundary invoke
1. `rc1 = sqlite3_reset(stmt)`
2. `rc2 = sqlite3_step(stmt)`  /* NO re-bind */
3. `v = sqlite3_column_int(stmt, 0)` /* bound value still present as implemented */

## Observables to capture
- `reset.rc`, `post_reset_step.rc`, `column_after_reset.value`

## Scrub
- None.
