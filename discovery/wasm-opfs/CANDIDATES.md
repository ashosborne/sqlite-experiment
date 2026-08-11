# CANDIDATES — wasm-opfs (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | OPFS async-proxy VFS | `ext/wasm/api/sqlite3-vfs-opfs.c-pp.js`, async worker `sqlite3-opfs-async-proxy.c-pp.js`, shared helpers `opfs-common-shared.c-pp.js` | Persistent browser storage via async OPFS proxied to sync VFS calls |
| 002 | OPFS SyncAccessHandle-pool VFS (sahpool) | `ext/wasm/api/sqlite3-vfs-opfs-sahpool.c-pp.js`, wl variant `sqlite3-vfs-opfs-wl.c-pp.js` | Alternative OPFS VFS without COOP/COEP requirements — different concurrency/limits |
