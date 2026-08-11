# wasm-opfs-001 — OPFS async-proxy VFS

Slice: `wasm-opfs` · Status: `documented` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Sync VFS calls proxied to an async OPFS worker; requires COOP/COEP; locking via OPFS handles.

## Entrypoints (citations)

- `sqlite3-vfs-opfs.c-pp.js` (other) — `ext/wasm/api/sqlite3-vfs-opfs.c-pp.js`, `ext/wasm/api/sqlite3-opfs-async-proxy.c-pp.js`, `ext/wasm/api/opfs-common-shared.c-pp.js`

## Inputs / outputs / observables

- VFS name 'opfs' when COOP/COEP headers + SharedArrayBuffer available; persistence across page reloads; async proxy worker spawned

## Behaviour (as implemented)

- sqlite3-vfs-opfs.c-pp.js installs a sync-looking VFS that proxies to an async OPFS worker (sqlite3-opfs-async-proxy.c-pp.js) via SharedArrayBuffer+Atomics; shared helpers opfs-common-*.c-pp.js

## Validation rules found in code

- Feature-detection refuses installation without isolation headers

## Edge cases found in code

- Lock contention across tabs mediated by OPFS access handles

## Dependencies

- wasm-js-api

## Assumptions / unknowns

- Confidence inferred; deployment-variant SME question stands

## Evidence

- `ext/wasm/api/sqlite3-vfs-opfs.c-pp.js`
- `ext/wasm/api/sqlite3-opfs-async-proxy.c-pp.js`
- `ext/wasm/api/opfs-common-shared.c-pp.js`
