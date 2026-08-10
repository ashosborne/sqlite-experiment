# Seed — backup-api
SLICE_ID: backup-api
SLICE_SEED: "online backup API (sqlite3_backup_*)"
SEED_ENTRYPOINTS: sqlite3_backup_init(), sqlite3_backup_step(), sqlite3_backup_finish()
OUT_OF_SCOPE: VACUUM INTO (vacuum seed), serialize/deserialize (serialize-memdb-api)
Rationale: self-contained page-copy subsystem with clear progress observables.
