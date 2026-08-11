# misc-cksumvfs-001 — cksumvfs per-page checksums

Slice: `misc-cksumvfs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

8-byte per-page checksums in reserve bytes; verification on read — FILE-FORMAT IMPACT (reserve-bytes requirement)

## Entrypoints (citations)

- `sqlite3_cksumvfs_init()` (other) — `ext/misc/cksumvfs.c:833`

## Inputs / outputs / observables

- 8-byte per-page checksums stored in page reserve space; verification on read (SQLITE_IOERR_DATA on mismatch); PRAGMA checksum_verification to query/toggle

## Behaviour (as implemented)

- init ext/misc/cksumvfs.c:833: shim computes/verifies checksums in xWrite/xRead when the db has 8 reserve bytes; file_control to enable on new dbs

## Validation rules found in code

- Requires reserve-bytes=8 (set at creation or via VACUUM after file_control)

## Edge cases found in code

- Databases created with cksumvfs are readable by ordinary builds (reserve bytes ignored) — but not verified; FILE-FORMAT deployment constraint stands

## Dependencies

- vfs-os-abstraction
- loadext-api

## Assumptions / unknowns

- Databases created with it are format-constrained — deployed anywhere?

## Evidence

- `ext/misc/cksumvfs.c:833`
