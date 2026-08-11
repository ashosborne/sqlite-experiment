# vfs-os-abstraction-002 — Unix VFS implementation

Slice: `vfs-os-abstraction` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

unixOpen + locking-style variants + shared-memory for WAL.

## Entrypoints (citations)

- `unixOpen()` (other) — `src/os_unix.c:6519`, `src/os_unix.c:8472`, `src/os_unix.c:4952`

## Inputs / outputs / observables

- POSIX advisory-lock behaviour; SQLITE_BUSY patterns across processes; shm files for WAL

## Behaviour (as implemented)

- unixOpen (src/os_unix.c:6519) + ioMethods selected per locking style; default posix byte-range locks on a lock-byte page; unixOpenSharedMemory (src/os_unix.c:4952) maps -shm for WAL coordination; vfs table registered at sqlite3_os_init (src/os_unix.c:8472 region defines unixVfs entries incl. 'unix-none', 'unix-dotfile', 'unix-excl')

## Validation rules found in code

- POSIX-lock close bug mitigations (open-file tracking per inode)

## Edge cases found in code

- 'unix-excl' takes exclusive locks avoiding shm files; threads sharing an fd interplay with posix locks (documented hazards)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/os_unix.c:6519`
- `src/os_unix.c:8472`
- `src/os_unix.c:4952`
