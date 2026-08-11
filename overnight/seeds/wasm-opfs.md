# Seed — wasm-opfs (resume run 2)
SLICE_ID: wasm-opfs
SLICE_SEED: "OPFS persistence variants: async-proxy VFS and SyncAccessHandle pool VFS"
SEED_ENTRYPOINTS: ext/wasm/api/sqlite3-vfs-opfs.c-pp.js, sqlite3-vfs-opfs-sahpool.c-pp.js
OUT_OF_SCOPE: kvvfs (vfs-kv), JS API layers (wasm-js-api)
Rationale: run-1 residual 5 — OPFS variants named but not carded.
