# prepare-statement-api-002-C003 — step a finalized statement

Feature: `prepare-statement-api-002` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: card Validation section ("Stepping a finalized
statement → SQLITE_MISUSE"), `src/vdbeapi.c:980`

## Preconditions / fixtures
- A prepared statement that has been passed to `sqlite3_finalize()`.

## Boundary invoke
1. `sqlite3_finalize(stmt)`
2. `rc = sqlite3_step(stmt)`  /* handle already finalized */

## Observables to capture
- `step_after_finalize.rc`

## Honest caveat (recorded for Test execution review)
- After finalize the handle memory is freed; the guarded-MISUSE answer is reliable when the build
  compiles the safety checks (`SQLITE_ENABLE_API_ARMOR`); without armor this is
  use-after-free territory and the observable may be undefined rather than a stable rc.
- Action for RECORD: capture the rc on the pinned baseline build AND note whether API_ARMOR is in
  the compileoption fingerprint. If the baseline lacks armor, Test execution should classify this
  case `BLOCKED (unsafe capture on unarmored build)` rather than freeze a golden from UB.

## Scrub
- None.
