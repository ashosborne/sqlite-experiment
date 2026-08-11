# blob-io-api-001 — Blob handle open/close/reopen

Slice: `blob-io-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Opens a handle on (db,table,column,rowid); reopen repositions to a new row without re-resolving.

## Entrypoints (citations)

- `sqlite3_blob_open()` (api) — `src/vdbeblob.c:121`, `src/vdbeblob.c:360`, `src/vdbeblob.c:503`, `src/sqlite.h.in:8206`

## Inputs / outputs / observables

- Return code + sqlite3_blob* handle; blob_bytes size; error message on the db handle

## Behaviour (as implemented)

- blob_open (src/vdbeblob.c:121) positions an internal statement on (db,table,column,rowid); flags!=0 opens read-write
- blob_reopen (src/vdbeblob.c:503) moves the handle to another rowid without re-checking schema (cheaper than reopen)
- blob_close (src/vdbeblob.c:360) finalizes; returns any deferred I/O error

## Validation rules found in code

- Cannot open on: view, virtual table, indexed/PK column in some modes, or column with FK/index constraints for write handles — SQLITE_ERROR with message
- Nonexistent row → SQLITE_ERROR 'no such rowid'

## Edge cases found in code

- Opening read-write on a column that is part of an index is refused (would bypass index maintenance)

## Dependencies

- btree
- vdbe-engine

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vdbeblob.c:121`
- `src/vdbeblob.c:360`
- `src/vdbeblob.c:503`
- `src/sqlite.h.in:8206`
