# misc-vfslog-001 — vfslog binary operation logger

Slice: `misc-vfslog` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Logs VFS operations to a binary file for later analysis; registered via sqlite3_register_vfslog (SQLITE_EXTRA_INIT hook)

## Entrypoints (citations)

- `sqlite3_register_vfslog()` (other) — `ext/misc/vfslog.c:755`, `ext/misc/vfslog.c:40`

## Inputs / outputs / observables

- Binary log file of VFS operations (op, file, offset, size, timing); vfslog vtab reads the log for analysis

## Behaviour (as implemented)

- sqlite3_register_vfslog (ext/misc/vfslog.c:755; SQLITE_EXTRA_INIT wiring :40): registers a logging VFS writing fixed-format records; companion vtab decodes

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Log grows unboundedly; intended for replay/analysis workflows

## Dependencies

- vfs-os-abstraction
- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/vfslog.c:755`
- `ext/misc/vfslog.c:40`
