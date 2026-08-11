# vfs-win-001 — Win32 VFS registration and file open

Slice: `vfs-win` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

winVfs registered at os_init; winOpen with win32 sharing/locking semantics; refines run-1 vfs-os-abstraction-003.

## Entrypoints (citations)

- `winOpen()` (other) — `src/os_win.c:5210`, `src/os_win.c:5211`, `src/os_win.c:3063`

## Inputs / outputs / observables

- Windows file sharing/locking semantics (LockFileEx region locks over the same lock-byte range as unix); UTF-16 filename handling; winVfs in vfs list

## Behaviour (as implemented)

- sqlite3_os_init (src/os_win.c:5210) registers winVfs (:5211) (+ variants longpath/none); winOpen (:3063) opens with FILE_SHARE_READ|WRITE, retry logic for AV-locked files (winIoerrRetry), temp-file handling in GetTempPath

## Validation rules found in code

- Path conversion ANSI/UTF-16 per build; long-path prefixes on longpath vfs

## Edge cases found in code

- Mandatory-vs-advisory lock differences vs unix are masked by using the same lock-byte protocol via region locks

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Windows-scope SME question stands
- Windows in migration scope?

## Evidence

- `src/os_win.c:5210`
- `src/os_win.c:5211`
- `src/os_win.c:3063`
