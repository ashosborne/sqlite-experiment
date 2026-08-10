# Seed — wasm-binding
SLICE_ID: wasm-binding
SLICE_SEED: "WASM/JS binding (sqlite3.js API, OPFS VFS, fiddle app)"
SEED_ENTRYPOINTS: ext/wasm/api/sqlite3-wasm.c exports
OUT_OF_SCOPE: emscripten build tooling (mkwasmbuilds.c, libcmpp.c — skipped-with-reason)
