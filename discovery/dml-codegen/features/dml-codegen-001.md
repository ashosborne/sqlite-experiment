# dml-codegen-001 — DML statement compilation (3 write paths)

Slice: `dml-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

INSERT (incl. SELECT-source + xfer optimization), UPDATE, DELETE (incl. truncate fast path).

## Entrypoints (citations)

- `sqlite3Insert()` (other) — `src/insert.c:900`, `src/update.c:285`, `src/delete.c:288`, `src/insert.c:3079`

## Inputs / outputs / observables

- changes()/last_insert_rowid() effects; rows inserted/updated/deleted; RETURNING output (dialect)

## Behaviour (as implemented)

- sqlite3Insert (src/insert.c:900): VALUES rows or SELECT source (co-routine), rowid allocation (max+1 or random after wraparound), defaults applied; xfer optimization (src/insert.c:3079) block-copies whole tables when schemas match exactly (INSERT INTO t SELECT * FROM s)
- sqlite3Update (src/update.c:285): one-pass vs two-pass strategies; sqlite3DeleteFrom (src/delete.c:288): truncate fast path when no WHERE/triggers/FKs (count via changes still correct)

## Validation rules found in code

- Column-count mismatch errors; 'rowid' insert of NULL auto-allocates

## Edge cases found in code

- Truncate optimization skipped when count_changes/triggers/FK need per-row work
- AUTOINCREMENT consults sqlite_sequence and forbids rowid reuse (max-rowid exhaustion → SQLITE_FULL)

## Dependencies

- btree
- expr-codegen

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/insert.c:900`
- `src/update.c:285`
- `src/delete.c:288`
- `src/insert.c:3079`
