# wasm-js-api-003 — worker1 message protocol + promiser

Slice: `wasm-js-api` · Status: `needs-SME` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

> **NEEDS-SME**: needs-SME: inferred-confidence card requires SME sign-off or waive-characterization before Test gen

## Summary

postMessage-based API for worker-thread use; promise wrapper. Async seam — split cards at bind.

## Entrypoints (citations)

- `ext/wasm/api/sqlite3-api-worker1.c-pp.js` (other) — `ext/wasm/api/sqlite3-api-worker1.c-pp.js`

## Inputs / outputs / observables

- worker1 postMessage protocol (open/exec/close/config-get messages with messageId correlation); promiser wrapper returning Promises

## Behaviour (as implemented)

- sqlite3-api-worker1.c-pp.js exposes a message-based API for running the db in a Worker; promiser JS builds a Promise API over the message protocol (async seam — run-2 flag)

## Validation rules found in code

- Errors returned as result messages with error info, not thrown across the boundary

## Edge cases found in code

- One-db-per-worker model; transfer of result rows via structured clone

## Dependencies

- wasm-js-api-002

## Assumptions / unknowns

- Confidence inferred; async protocol card-split flag stands for Test gen

## Evidence

- `ext/wasm/api/sqlite3-api-worker1.c-pp.js`
