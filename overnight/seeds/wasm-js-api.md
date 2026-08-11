# Seed — wasm-js-api (resume run 2)
SLICE_ID: wasm-js-api
SLICE_SEED: "wasm JS API breadth: oo1, worker1, promiser, C-glue beyond the run-1 umbrella"
SEED_ENTRYPOINTS: ext/wasm/api/sqlite3-api-oo1.c-pp.js, sqlite3-api-worker1.c-pp.js, sqlite3-api-prologue.js
OUT_OF_SCOPE: OPFS persistence (wasm-opfs), emscripten build tooling (still skipped-with-reason)
Rationale: run-1 residual 5 — only the C-glue seam was evidenced; the layered JS APIs were not carded.
