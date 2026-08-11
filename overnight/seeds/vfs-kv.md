# Seed — vfs-kv (resume run 2)
SLICE_ID: vfs-kv
SLICE_SEED: "key-value VFS backend (src/os_kv.c)"
SEED_ENTRYPOINTS: sqlite3KvvfsInit(), kvvfsOpen()
OUT_OF_SCOPE: wasm JS glue that consumes it (wasm-opfs / wasm-js-api)
Rationale: run-1 residual 4 — os_kv.c inventoried but not seeded.
