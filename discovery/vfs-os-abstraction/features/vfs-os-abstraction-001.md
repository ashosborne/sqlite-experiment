# vfs-os-abstraction-001 — VFS registration and lookup

Slice: `vfs-os-abstraction` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Register/find/unregister VFS list with default selection.

## Entrypoints (citations)

- `sqlite3_vfs_register()` (api) — `src/os.c:408`, `src/os.c:362`, `src/sqlite.h.in:8381`

## Inputs / outputs / observables

- sqlite3_vfs_find results; default VFS selection; registration ordering

## Behaviour (as implemented)

- sqlite3_vfs_register (src/os.c:408) links into the global VFS list (makeDflt moves to head); vfs_find (src/os.c:362) by name or default; sqlite3OsOpen (src/os.c:215) dispatches file ops through the chosen vfs's method table

## Validation rules found in code

- Unregistering the default promotes the next in list

## Edge cases found in code

- VFS name lookup is exact-match; URI vfs= parameter overrides per-open

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/os.c:408`
- `src/os.c:362`
- `src/sqlite.h.in:8381`
