# wasm-opfs-002 — OPFS SyncAccessHandle-pool VFS (sahpool)

Slice: `wasm-opfs` · Status: `needs-SME` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

> **NEEDS-SME**: needs-SME: inferred-confidence card requires SME sign-off or waive-characterization before Test gen

## Summary

Pre-acquired SAH pool; no COOP/COEP; capacity-limited filenames; wl variant present.

## Entrypoints (citations)

- `sqlite3-vfs-opfs-sahpool.c-pp.js` (other) — `ext/wasm/api/sqlite3-vfs-opfs-sahpool.c-pp.js`, `ext/wasm/api/sqlite3-vfs-opfs-wl.c-pp.js`

## Inputs / outputs / observables

- 'opfs-sahpool' VFS: no COOP/COEP requirement; fixed-capacity pool of pre-acquired SyncAccessHandles; filename mapping layer

## Behaviour (as implemented)

- sqlite3-vfs-opfs-sahpool.c-pp.js acquires N OPFS sync handles up front and maps SQLite filenames onto them (opaque names + metadata); wl variant (sqlite3-vfs-opfs-wl.c-pp.js) adds write-lock behaviour

## Validation rules found in code

- Pool exhaustion → open failures; capacity configurable at install

## Edge cases found in code

- Single-tab exclusivity by default (handles are exclusive) — different concurrency model vs 'opfs'

## Dependencies

- wasm-opfs-001

## Assumptions / unknowns

- Confidence inferred
- Which OPFS variant does downstream deploy? Concurrency semantics differ.

## Evidence

- `ext/wasm/api/sqlite3-vfs-opfs-sahpool.c-pp.js`
- `ext/wasm/api/sqlite3-vfs-opfs-wl.c-pp.js`
