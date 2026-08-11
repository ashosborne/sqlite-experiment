# misc-appendvfs-001 — appendvfs (db appended to host file)

Slice: `misc-appendvfs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Opens a db image appended to another file (e.g. self-contained executable) with marker + offset handling

## Entrypoints (citations)

- `sqlite3_appendvfs_init()` (other) — `ext/misc/appendvfs.c:651`

## Inputs / outputs / observables

- VFS 'apndvfs'; opening a db appended to any host file (e.g. executable) via vfs=apndvfs; marker 'Start-Of-SQLite3-' + offset trailer

## Behaviour (as implemented)

- init ext/misc/appendvfs.c:651: detects the append marker at file end, translates all page I/O by the stored offset; can create appended dbs onto existing files

## Validation rules found in code

- Host file without marker: creates one if opened for write with create intent; size limits for marker search window

## Edge cases found in code

- The shell's -append flag uses this; db grows the host file in place

## Dependencies

- vfs-os-abstraction
- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/appendvfs.c:651`
