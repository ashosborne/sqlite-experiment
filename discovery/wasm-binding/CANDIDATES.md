# CANDIDATES — wasm-binding (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | WASM/JS API layer + OPFS persistence | C glue `ext/wasm/api/sqlite3-wasm.c` (SQLITE_WASM_EXPORT `:236`, extra-init hook `:145`), JS API `ext/wasm/api/sqlite3-api-glue.c-pp.js`, OO API `sqlite3-api-oo1.c-pp.js`, OPFS glue `opfs-common-shared.c-pp.js` | Browser adapter incl. persistent OPFS VFS; fiddle app is the demo UI (deprioritized) |
