# blob-io-api-002 — Incremental read/write with bounds + expiry

Slice: `blob-io-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

read/write at offset via shared blobReadWrite; handles expire on row modification (SQLITE_ABORT).

## Entrypoints (citations)

- `sqlite3_blob_read()` (api) — `src/vdbeblob.c:471`, `src/vdbeblob.c:478`, `src/vdbeblob.c:381`, `src/vdbeblob.c:488`

## Inputs / outputs / observables

- SQLITE_OK/SQLITE_ERROR/SQLITE_ABORT return codes; bytes read/written at offset

## Behaviour (as implemented)

- read/write (src/vdbeblob.c:471,478) share blobReadWrite (src/vdbeblob.c:381) → btree payload access at offset without loading the whole cell
- Write cannot change blob size (no append) — size fixed at row creation (use zeroblob to preallocate)

## Validation rules found in code

- offset+n beyond blob → SQLITE_ERROR (no partial transfer)
- Write on read-only handle → SQLITE_READONLY

## Edge cases found in code

- Handle expires (SQLITE_ABORT) when its row is modified by any statement — including the same connection

## Dependencies

- blob-io-api-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vdbeblob.c:471`
- `src/vdbeblob.c:478`
- `src/vdbeblob.c:381`
- `src/vdbeblob.c:488`
