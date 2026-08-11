# loadext-api-001 — Load extension with enable gate

Slice: `loadext-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

dlopens shared lib, derives sqlite3_X_init entry name from filename; disabled by default (enable_load_extension gate).

## Entrypoints (citations)

- `sqlite3_load_extension()` (api) — `src/loadext.c:731`, `src/loadext.c:561`, `src/loadext.c:762`, `src/sqlite.h.in:7609`

## Inputs / outputs / observables

- Return code + optional error message (dlopen error text); extension entry point invoked with sqlite3_api_routines

## Behaviour (as implemented)

- sqlite3_load_extension (src/loadext.c:731) → sqlite3LoadExtension (src/loadext.c:561): dlopen the file, derive entry name sqlite3_<basename>_init (strip lib prefix/dots), call it with the API-routines thunk
- Disabled by default: both C API gate (enable_load_extension src/loadext.c:762) and SQL load_extension() function gated separately (db_config DBCONFIG_ENABLE_LOAD_EXTENSION distinguishes them)

## Validation rules found in code

- Entry point returning non-zero → SQLITE_ERROR + message; missing symbol → tries legacy sqlite3_extension_init

## Edge cases found in code

- Extension loaded into one connection only; persistent extensions must return SQLITE_OK_LOAD_PERMANENTLY

## Dependencies

- vfs-os-abstraction

## Assumptions / unknowns

- Security posture question from run 1 stands (dlopen from SQL-adjacent surface)
- Security posture: is runtime loading allowed in target at all?

## Evidence

- `src/loadext.c:731`
- `src/loadext.c:561`
- `src/loadext.c:762`
- `src/sqlite.h.in:7609`
