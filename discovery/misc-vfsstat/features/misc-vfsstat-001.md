# misc-vfsstat-001 — vfsstat I/O statistics shim + vtab

Slice: `misc-vfsstat` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Counts read/write/sync per file class; exposes vfsstat vtab for SQL queries

## Entrypoints (citations)

- `sqlite3_vfsstat_init()` (other) — `ext/misc/vfsstat.c:806`

## Inputs / outputs / observables

- SELECT * FROM vfsstat — (file-class, stat, count) rows for read/write/sync/open/lock... ; vstat() reset semantics

## Behaviour (as implemented)

- init ext/misc/vfsstat.c:806: shim VFS counts operations by file class (main/journal/wal/temp...); vtab reads the counter table

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Counters process-global for the shim instance

## Dependencies

- vfs-os-abstraction
- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/vfsstat.c:806`
