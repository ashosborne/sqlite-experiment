# prepare-statement-api-001-C001 — prepare_v2 of a valid single statement

Feature: `prepare-statement-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/prepare.c:955`, `src/prepare.c:700`, `src/prepare.c:854`

## Preconditions / fixtures
- Pinned baseline build (overnight/BASELINE.md, run-5 fingerprint is law). Fresh `:memory:` connection.

## Inputs
- SQL: `"SELECT 1"`, nByte=-1, with pzTail out-param supplied.

## Boundary invoke
1. `rc = sqlite3_prepare_v2(db, "SELECT 1", -1, &stmt, &zTail)`

## Observables to capture
- `prepare.rc`
- `stmt.nonnull` (1/0 — handle produced)
- `pzTail.consumed` (1 if zTail points at the terminating NUL / end of input; else the unconsumed text)

## Scrub
- None (static input).
