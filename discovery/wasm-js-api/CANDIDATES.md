# CANDIDATES — wasm-js-api (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Refines run-1 wasm-binding-001 (row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Low-level JS API + C-glue (capi namespace) | `ext/wasm/api/sqlite3-api-prologue.js`, `sqlite3-api-glue.c-pp.js`, exports list `EXPORTED_FUNCTIONS.c-pp` | 1:1 C-API projection into JS |
| 002 | oo1 object-oriented JS API | `ext/wasm/api/sqlite3-api-oo1.c-pp.js` | High-level DB/Stmt classes — the API most JS consumers use |
| 003 | worker1 + promiser message API | `ext/wasm/api/sqlite3-api-worker1.c-pp.js` | Worker-thread message protocol (async seam — flagged) |
