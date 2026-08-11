# vfs-kv-002 — Pluggable K/V storage method table

Slice: `vfs-kv` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_kvvfs_methods xRead/xWrite/xDelete callbacks replaceable at runtime.

## Entrypoints (citations)

- `sqlite3_kvvfs_methods` (other) — `src/os_kv.c:330`, `src/os_kv.c:361`

## Inputs / outputs / observables

- Replaceable read/write/delete callbacks (sqlite3_kvvfs_methods) redirecting persistence

## Behaviour (as implemented)

- Method table (src/os_kv.c:330, instance :361) defaults to in-memory/file-shim on native builds; wasm build repoints it at localStorage/sessionStorage JS shims

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Callback errors surface as SQLITE_IOERR family

## Dependencies

- vfs-kv-001

## Assumptions / unknowns

- Non-wasm consumers question stands
- Non-wasm consumers exist?

## Evidence

- `src/os_kv.c:330`
- `src/os_kv.c:361`
