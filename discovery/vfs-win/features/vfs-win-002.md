# vfs-win-002 — Windows shared-memory and mmap paths (WAL)

Slice: `vfs-win` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

winShmMap implements wal-index shm on win32; winFetch mmap read path.

## Entrypoints (citations)

- `winShmMap()` (other) — `src/os_win.c:3586`, `src/os_win.c:3723`, `src/os_win.c:3876`

## Inputs / outputs / observables

- WAL on Windows via shared memory (winShmMap); mmap read path (winFetch) under mmap_size pragma

## Behaviour (as implemented)

- winShmMap (src/os_win.c:3586) implements the shm region over a mapped -shm file with the same locking slots as unix (no-shm stub :3723 when gated); winFetch/winUnfetch (:3876) provide xFetch mmap reads

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Windows cannot unlink open files — shm/wal cleanup ordering differs from unix (visible in file lifecycle)

## Dependencies

- wal

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/os_win.c:3586`
- `src/os_win.c:3723`
- `src/os_win.c:3876`
