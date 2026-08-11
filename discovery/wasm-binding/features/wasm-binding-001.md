# wasm-binding-001 — WASM/JS API layer with OPFS persistence

Slice: `wasm-binding` · Status: `needs-SME` · Confidence: `inferred` · Card written: 2026-08-11T10:32:22Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

> **NEEDS-SME**: needs-SME: inferred-confidence card requires SME sign-off or waive-characterization before Test gen

## Summary

C export glue + layered JS APIs (low-level, oo1) + OPFS-backed VFS for browser persistence; fiddle UI noted as adapter, deprioritized.

## Entrypoints (citations)

- `ext/wasm/api/sqlite3-wasm.c` (other) — `ext/wasm/api/sqlite3-wasm.c:236`, `ext/wasm/api/sqlite3-wasm.c:145`, `ext/wasm/api/sqlite3-api-glue.c-pp.js`, `ext/wasm/api/sqlite3-api-oo1.c-pp.js`

## Inputs / outputs / observables

- sqlite3.js module exports; SQLITE_WASM_EXPORT-decorated C helpers callable from JS; extra-init hook wiring

## Behaviour (as implemented)

- ext/wasm/api/sqlite3-wasm.c provides wasm-specific C utilities (export macro :236; SQLITE_EXTRA_INIT hook :145) compiled with the amalgamation for the JS build; JS layers (glue/oo1/worker1 — see wasm-js-api cards) sit on top; kvvfs (vfs-kv cards) and OPFS (wasm-opfs cards) provide persistence

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Umbrella card: deep contracts live in wasm-js-api-00x / wasm-opfs-00x / vfs-kv-00x cards

## Dependencies

- vfs-os-abstraction
- serialize-memdb-api

## Assumptions / unknowns

- Confidence inferred at umbrella level (build-product behaviour not executed this run); browser-scope SME question stands
- Browser target in migration scope at all?

## Evidence

- `ext/wasm/api/sqlite3-wasm.c:236`
- `ext/wasm/api/sqlite3-wasm.c:145`
- `ext/wasm/api/sqlite3-api-glue.c-pp.js`
- `ext/wasm/api/sqlite3-api-oo1.c-pp.js`
