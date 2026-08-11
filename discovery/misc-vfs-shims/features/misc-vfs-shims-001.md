# misc-vfs-shims-001 — Stackable VFS shim family (9 extensions)

Slice: `misc-vfs-shims` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:32:22Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

appendvfs, cksumvfs, vfsstat, vfstrace, vfslog, tmstmpvfs, mmapwarm, memtrace, pcachetrace — wrappers over a lower VFS.

## Entrypoints (citations)

- `ext/misc VFS shims` (other) — `ext/misc/appendvfs.c:651`, `ext/misc/cksumvfs.c:833`, `ext/misc/vfsstat.c:806`, `ext/misc/tmstmpvfs.c:1029`

## Inputs / outputs / observables

- Per-shim registration effects (named VFS appears in vfs_find); wrapped-VFS pass-through with the shim's specific side effects

## Behaviour (as implemented)

- Cluster card refined by 6 thin run-2 slices: misc-appendvfs-001, misc-cksumvfs-001, misc-vfsstat-001, misc-vfstrace-001, misc-vfslog-001, misc-tmstmpvfs-001 (see their cards); remaining members mmapwarm/memtrace/pcachetrace are still unscanned hints — NOT carded (honest residual)
- Common shape: sqlite3_vfs wrapper delegating xOpen/xRead/... to a parent VFS with interposed behaviour

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- (none found in code)

## Dependencies

- vfs-os-abstraction

## Assumptions / unknowns

- Umbrella retained for inventory continuity; per-shim contracts in thin cards; 3 residual members excluded by design
- Which shims are actually deployed downstream? cksumvfs changes the file format (reserve bytes)

## Evidence

- `ext/misc/appendvfs.c:651`
- `ext/misc/cksumvfs.c:833`
- `ext/misc/vfsstat.c:806`
- `ext/misc/tmstmpvfs.c:1029`
