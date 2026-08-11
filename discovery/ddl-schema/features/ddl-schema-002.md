# ddl-schema-002 — Index create-drop lifecycle

Slice: `ddl-schema` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

CREATE INDEX incl. UNIQUE, partial (WHERE), expression indexes; backfill on creation.

## Entrypoints (citations)

- `sqlite3CreateIndex()` (other) — `src/build.c:3974`, `src/build.c:4629`

## Inputs / outputs / observables

- Index rows in sqlite_master; UNIQUE violation error on backfill; 'partial index WHERE clause' restrictions

## Behaviour (as implemented)

- sqlite3CreateIndex (src/build.c:3974) validates columns/expressions, allocates the index btree and backfills by scanning the table (sorter-fed); UNIQUE enforced during backfill; DropIndex (src/build.c:4629) removes
- Expression and partial (WHERE) indexes recorded with their expression trees; collation per column

## Validation rules found in code

- Partial-index WHERE must be deterministic and reference only the table's columns
- Descending index columns honored per file-format version

## Edge cases found in code

- Implicit indexes from UNIQUE constraints cannot be dropped by name (autoindex)
- CREATE INDEX on a view/vtab → error

## Dependencies

- btree
- where-optimizer

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/build.c:3974`
- `src/build.c:4629`
