# Seed — error-status-api
SLICE_ID: error-status-api
SLICE_SEED: "error introspection + limits + status counters"
SEED_ENTRYPOINTS: sqlite3_errmsg(), sqlite3_errcode(), sqlite3_limit(), sqlite3_status64(), sqlite3_db_status()
OUT_OF_SCOPE: error propagation internals in each subsystem
Rationale: read-mostly observability seam used by every consumer; thin and testable.
