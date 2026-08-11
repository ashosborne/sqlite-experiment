# prepare-statement-api-002-C001 — step ROW→DONE sequence on a single-row SELECT

Feature: `prepare-statement-api-002` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/vdbeapi.c:980`, `src/sqlite.h.in:5347`

## Preconditions / fixtures
- Baseline build (overnight/BASELINE.md). Fresh `:memory:` connection.

## Inputs
- SQL: `"SELECT 1"` (deterministic single row, no schema fixtures).

## Boundary invoke
1. `rc0 = sqlite3_prepare_v2(db, "SELECT 1", -1, &stmt, NULL)`
2. `rc1 = sqlite3_step(stmt)` (expected-shape ROW — recorded, not asserted)
3. `v = sqlite3_column_int(stmt, 0)` (row observable)
4. `rc2 = sqlite3_step(stmt)` (expected-shape DONE — recorded, not asserted)

## Observables to capture
- `prepare.rc`, `step1.rc`, `column_int.value`, `step2.rc`

## Scrub
- None.
