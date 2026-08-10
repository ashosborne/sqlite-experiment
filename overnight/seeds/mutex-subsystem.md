# Seed — mutex-subsystem
SLICE_ID: mutex-subsystem
SLICE_SEED: "mutex abstraction (pluggable methods, noop/unix/win impls)"
SEED_ENTRYPOINTS: sqlite3_mutex_alloc()
OUT_OF_SCOPE: btree/connection-level locking policies
