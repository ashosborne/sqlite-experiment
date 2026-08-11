# engine-kitchen-001 — Kitchen-spine row round-trip

Slice: `engine-kitchen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T15:29:34Z
Composed slice (run 13): a thin cross-section of ddl-schema/dml-codegen/select-codegen behaviour,
carved out so pack v4 can require a REAL row store on the Rust side (no recognizer for these shapes).

## Summary
CREATE TABLE (INTEGER/TEXT cols), INSERT VALUES (int/string literals, multi-row), UPDATE SET
col=literal|col+int [WHERE col=int], DELETE [WHERE col=int], SELECT cols FROM table [ORDER BY col],
plus count(*) over sqlite_master and changes()/total_changes() — as implemented by the pinned C library.

## Entrypoints (citations)
- `sqlite3_exec()` over the five kitchen scripts — src/build.c:1225 (CREATE), src/insert.c:900,
  src/update.c:285, src/delete.c:288, src/select.c:7642

## Inputs / outputs / observables
- exec.rc, callback rows (values as text) — frozen in the five engine-kitchen goldens

## Behaviour (as implemented)
- Rows persist in the connection; UPDATE/DELETE affect matching rows; SELECT returns current store
  content; ORDER BY integer column ascending; changes()/total_changes() count row effects.

## Assumptions / unknowns
- Deliberately excludes joins/indexes/views/FK/files (out_of_scope).

## Evidence
- The five frozen goldens under tests/characterization/engine-kitchen/ + citations above.
