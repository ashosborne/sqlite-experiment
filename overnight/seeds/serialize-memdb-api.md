# Seed — serialize-memdb-api
SLICE_ID: serialize-memdb-api
SLICE_SEED: "database serialize/deserialize + in-memory VFS (memdb)"
SEED_ENTRYPOINTS: sqlite3_serialize(), sqlite3_deserialize(), memdb_vfs
OUT_OF_SCOPE: file-backed VFS (vfs-os-abstraction), backup API
Rationale: single file src/memdb.c implements both the public serialize API and the memdb VFS it rides on.
