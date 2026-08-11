# wasm-js-api-001 — Low-level JS capi projection + C glue

Slice: `wasm-js-api` · Status: `documented` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3.capi namespace built by prologue/glue over exported C functions.

## Entrypoints (citations)

- `ext/wasm/api/sqlite3-api-prologue.js` (other) — `ext/wasm/api/sqlite3-api-prologue.js`, `ext/wasm/api/sqlite3-api-glue.c-pp.js`, `ext/wasm/api/EXPORTED_FUNCTIONS.c-pp`

## Inputs / outputs / observables

- sqlite3.capi.* functions mirroring the C API; sqlite3.wasm memory helpers (peek/poke/alloc); exported-function list

## Behaviour (as implemented)

- Prologue (ext/wasm/api/sqlite3-api-prologue.js) builds the namespace; glue (sqlite3-api-glue.c-pp.js) wraps exported C symbols (EXPORTED_FUNCTIONS.c-pp) with type conversion incl. string/pointer marshalling and function-pointer binding for callbacks

## Validation rules found in code

- Argument-type adapters throw JS exceptions on misuse (converted from C MISUSE where possible)

## Edge cases found in code

- Pointer-passing interfaces (e.g. fts5_api) use the 'pointer-argument' convention

## Dependencies

- wasm-binding

## Assumptions / unknowns

- Confidence inferred: JS layer not executed/traced this run; file-level evidence

## Evidence

- `ext/wasm/api/sqlite3-api-prologue.js`
- `ext/wasm/api/sqlite3-api-glue.c-pp.js`
- `ext/wasm/api/EXPORTED_FUNCTIONS.c-pp`
