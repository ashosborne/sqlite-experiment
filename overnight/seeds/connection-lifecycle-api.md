# Seed — connection-lifecycle-api
SLICE_ID: connection-lifecycle-api
SLICE_SEED: "database connection open/close/configure lifecycle (sqlite3 handle)"
SEED_ENTRYPOINTS: sqlite3_open(), sqlite3_open_v2(), sqlite3_close(), sqlite3_busy_timeout(), sqlite3_trace_v2()
OUT_OF_SCOPE: statement lifecycle (prepare-statement-api), error/status introspection (error-status-api), library-global init/config (kept as open question), ATTACH (attach-detach)
Rationale: thin callable boundary at the top of the public API; observable outcomes (handle state, URI parsing, busy callbacks). Not a mega-slice: excludes statement + error seams.
