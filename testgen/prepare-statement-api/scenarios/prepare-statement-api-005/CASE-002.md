# prepare-statement-api-005-C002 — finalize a live statement (finalize.rc only)

Feature: `prepare-statement-api-005` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/vdbeapi.c:105` (finalize returns last error; legal in any state)

## Preconditions / fixtures
- Fresh prepared `"SELECT 1"`, stepped once to ROW (live, un-finalized, mid-row).

## Boundary invoke
1. `rc = sqlite3_finalize(stmt)`

## Observables to capture
- `finalize.rc` **only**.

## Hard boundary (operator)
- This is NOT prepare-statement-api-002-C003. The harness must NOT touch the handle after finalize —
  no step, no column, nothing. The freed handle is dead.

## Scrub
- None.
