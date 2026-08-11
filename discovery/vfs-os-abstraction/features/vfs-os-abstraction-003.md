# vfs-os-abstraction-003 — Windows and KV platform variants

Slice: `vfs-os-abstraction` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

os_win.c full win32 VFS; os_kv.c key-value VFS (wasm/localStorage backend).

## Entrypoints (citations)

- `src/os_win.c / src/os_kv.c` (other) — `src/os_win.c`, `src/os_kv.c`

## Inputs / outputs / observables

- Platform selection at compile; win32 sharing/locking semantics; kv-backed persistence

## Behaviour (as implemented)

- os_win.c full win32 VFS (LockFileEx-based range locks, UTF-16 paths); os_kv.c key-value VFS for wasm/localStorage-class backends — both refined by run-2 slices vfs-win/vfs-kv (see their cards for line-cited detail)

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- (none found in code)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Umbrella row retained for inventory continuity; deep contracts live in vfs-win-00x / vfs-kv-00x cards
- Which platforms are in migration scope?

## Evidence

- `src/os_win.c`
- `src/os_kv.c`
- `src/os_win.c:5210`
- `src/os_kv.c:1094`
