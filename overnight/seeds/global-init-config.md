# Seed — global-init-config (resume run 2)
SLICE_ID: global-init-config
SLICE_SEED: "process-wide init/shutdown + global and per-db configuration"
SEED_ENTRYPOINTS: sqlite3_initialize(), sqlite3_config(), sqlite3_db_config()
OUT_OF_SCOPE: subsystem behaviour the knobs configure (own slices)
Rationale: left as an explicit open question in run-1 connection-lifecycle SME brief; now a slice.
