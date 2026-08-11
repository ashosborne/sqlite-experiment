# vfs-unix-variants-001 — Locking-style selection matrix

Slice: `vfs-unix-variants` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Compile+runtime selection among 8 locking strategies; auto-detection per filesystem when LOCKING_STYLE enabled.

## Entrypoints (citations)

- `SQLITE_ENABLE_LOCKING_STYLE paths` (other) — `src/os_unix.c:66`, `src/os_unix.c:61`, `src/os_unix.c:143`

## Inputs / outputs / observables

- Locking style per filesystem when SQLITE_ENABLE_LOCKING_STYLE: posix/flock/dotlock/sem/afp/nfs/proxy/none; style visible via failure modes on network filesystems

## Behaviour (as implemented)

- CENSUS CARD for the 8-style matrix (gate src/os_unix.c:66, docs :61,119, apple auto-detect :143): each style is an io_methods table with its own lock/unlock/check implementations; auto-selection probes the filesystem on Apple builds

## Validation rules found in code

- Styles 4/5/7 only with LOCKING_STYLE=1

## Edge cases found in code

- dotlock is not advisory-safe across crash (stale lockfile); flock has no shared-lock upgrade path

## Dependencies

- compile-options-omit-enable

## Assumptions / unknowns

- Default Linux baseline uses plain posix — matrix documented, not exploded
- Any NFS/AFP deployment downstream?

## Evidence

- `src/os_unix.c:66`
- `src/os_unix.c:61`
- `src/os_unix.c:143`
