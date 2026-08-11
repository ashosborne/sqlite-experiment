# wasm-js-api-002 — oo1 object-oriented JS API (DB/Stmt)

Slice: `wasm-js-api` · Status: `documented` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

High-level DB/Stmt wrapper with exec() config object semantics.

## Entrypoints (citations)

- `ext/wasm/api/sqlite3-api-oo1.c-pp.js` (other) — `ext/wasm/api/sqlite3-api-oo1.c-pp.js`

## Inputs / outputs / observables

- oo1.DB/Stmt API: exec() config object (sql, bind, rowMode, callback, resultRows), prepared-statement lifecycle, exceptions as SQLite3Error

## Behaviour (as implemented)

- sqlite3-api-oo1.c-pp.js implements the OO layer over capi with finalization discipline and transaction helpers; DB constructors accept filename+vfs (incl. kvvfs/opfs names)

## Validation rules found in code

- Bind-type mapping JS↔SQLite (BigInt for int64 when enabled)

## Edge cases found in code

- rowMode variants (array/object/$columnName) change callback shapes

## Dependencies

- wasm-js-api-001

## Assumptions / unknowns

- Confidence inferred; browser-scope SME question stands

## Evidence

- `ext/wasm/api/sqlite3-api-oo1.c-pp.js`
