# prepare-statement-api-001-C002 — whitespace/comment-only SQL

Feature: `prepare-statement-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/prepare.c:955`, `src/prepare.c:700`; run-4 card ("NULL for whitespace/comment-only SQL with SQLITE_OK" per testgen spec heritage)

## Preconditions / fixtures
- Same fresh `:memory:` connection pattern.

## Inputs
- SQL: `"  -- just a comment\n  "` (whitespace + line comment only), nByte=-1, pzTail supplied.

## Boundary invoke
1. `rc = sqlite3_prepare_v2(db, zSql, -1, &stmt, &zTail)`

## Observables to capture
- `prepare.rc` (card shape: SQLITE_OK)
- `stmt.isnull` (card shape: NULL statement — recorded as implemented, not asserted)
- `pzTail.consumed`

## Scrub
- None.
