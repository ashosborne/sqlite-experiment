# misc-tmstmpvfs-001 — timestamped-backup VFS

Slice: `misc-tmstmpvfs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Keeps timestamped copies of the database on write — simple point-in-time backups

## Entrypoints (citations)

- `sqlite3_tmstmpvfs_init()` (other) — `ext/misc/tmstmpvfs.c:1029`

## Inputs / outputs / observables

- Timestamped backup copies of the database file created on write-close cycles; naming pattern with time components

## Behaviour (as implemented)

- init ext/misc/tmstmpvfs.c:1029: shim VFS that snapshots the main db to timestamped copies per its policy (point-in-time recovery aid)

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Copy storms under frequent commits — policy-dependent

## Dependencies

- vfs-os-abstraction
- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/tmstmpvfs.c:1029`
