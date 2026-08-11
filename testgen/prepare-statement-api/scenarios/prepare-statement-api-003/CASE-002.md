# prepare-statement-api-003-C002 — bind index out of range

Feature: `prepare-statement-api-003` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: card Validation section ("Index out of range → SQLITE_RANGE"), `src/vdbeapi.c:1816`

## Preconditions / fixtures
- The SAME prepared `"SELECT ?"` statement as 003-C001, after `sqlite3_reset()` (exactly one parameter; harness reuses the C001 stmt post-reset — this spec matches that flow).

## Boundary invoke
1. `rc = sqlite3_bind_int(stmt, 2, 7)`  /* index 2 of 1 */

## Observables to capture
- `bind_oor.rc` (card shape: SQLITE_RANGE — recorded, not asserted)

## Scrub
- None.
