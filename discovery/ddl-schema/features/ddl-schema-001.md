# ddl-schema-001 — Table/view create-drop lifecycle

Slice: `ddl-schema` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

CREATE TABLE/VIEW parse-to-schema flow writing sqlite_master; IF NOT EXISTS, temp scoping.

## Entrypoints (citations)

- `sqlite3StartTable()` (other) — `src/build.c:1225`, `src/build.c:2670`, `src/build.c:3023`, `src/build.c:3528`

## Inputs / outputs / observables

- sqlite_master/sqlite_schema rows; schema_version bump; errors 'table X already exists', 'no such table', IF (NOT) EXISTS suppression

## Behaviour (as implemented)

- sqlite3StartTable/EndTable (src/build.c:1225,2670) build Table objects then write the CREATE text into sqlite_master and update the in-memory schema in the same transaction; CreateView (src/build.c:3023) stores SELECT text, columns resolved lazily; DropTable (src/build.c:3528) deletes rows, indices, triggers and the btree

## Validation rules found in code

- Reserved name prefix sqlite_ refused for user objects
- WITHOUT ROWID requires a PRIMARY KEY; STRICT tables enforce declared types (dialect gates)

## Edge cases found in code

- CREATE TABLE ... AS SELECT derives column affinities from the SELECT (no constraints)
- DROP of a view vs table cross-errors ('use DROP VIEW')

## Dependencies

- btree

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/build.c:1225`
- `src/build.c:2670`
- `src/build.c:3023`
- `src/build.c:3528`
