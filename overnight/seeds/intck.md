# Seed — intck
SLICE_ID: intck
SLICE_SEED: "incremental integrity check extension"
SEED_ENTRYPOINTS: sqlite3_intck_open(), sqlite3_intck_step()
OUT_OF_SCOPE: PRAGMA integrity_check (pragma-surface)
