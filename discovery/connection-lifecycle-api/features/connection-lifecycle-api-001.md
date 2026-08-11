# connection-lifecycle-api-001 — Open database (sqlite3_open family + URI parsing)

Slice: `connection-lifecycle-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_open/open16/open_v2 funnel into static openDatabase; URI filenames parsed by sqlite3ParseUri (vfs/mode/cache params).

## Entrypoints (citations)

- `sqlite3_open_v2()` (api) — `src/main.c:3749`, `src/main.c:3380`, `src/main.c:3125`, `src/sqlite.h.in:4027`

## Inputs / outputs / observables

- Return code (SQLITE_OK/SQLITE_CANTOPEN/SQLITE_MISUSE...) plus a non-NULL sqlite3* even on most failures (caller must sqlite3_close it)
- URI query params consumed: vfs=, mode=ro|rw|rwc|memory, cache=shared|private, immutable=, psow=, nolock=

## Behaviour (as implemented)

- All three variants funnel to static openDatabase (src/main.c:3380): allocate db handle, init mutex/lookaside, parse URI when SQLITE_USE_URI or OPEN_URI flag set (sqlite3ParseUri src/main.c:3125), select VFS, open main btree, register built-in functions/collations
- sqlite3_open == open_v2 with SQLITE_OPEN_READWRITE|CREATE; open16 converts filename from UTF-16 and sets db encoding UTF-16 native
- Auto-calls sqlite3_initialize; failure surfaces as the open return code

## Validation rules found in code

- open_v2 asserts legal flag combinations (READONLY/READWRITE/CREATE trio); illegal combos → SQLITE_MISUSE
- mode= URI param may not grant more access than open flags (error 'access mode not allowed')
- Unknown query parameters ignored unless vfs= names a missing VFS → SQLITE_ERROR with 'no such vfs'

## Edge cases found in code

- Filename ":memory:" (or mode=memory) opens in-memory db — no file
- Empty filename opens a private temporary on-disk db deleted on close
- URI parsing only if sqlite3_config(SQLITE_CONFIG_URI,1) or OPEN_URI flag; otherwise literal filename incl. '?'
- OOM during open still returns a handle with db->mallocFailed set (errcode SQLITE_NOMEM)

## Dependencies

- vfs-os-abstraction
- malloc-subsystem

## Assumptions / unknowns

- Default build: URI handling compile-default SQLITE_USE_URI is off in this tree unless configure enables it — fingerprint via baseline pin
- Which open flags combinations are legal per compile-time options?

## Evidence

- `src/main.c:3749`
- `src/main.c:3380`
- `src/main.c:3125`
- `src/sqlite.h.in:4027`
