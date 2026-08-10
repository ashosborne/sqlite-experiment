# Seed — rbu
SLICE_ID: rbu
SLICE_SEED: "RBU: resumable bulk update / vacuum over OTA-style delta files"
SEED_ENTRYPOINTS: sqlite3rbu_open(), sqlite3rbu_step()
OUT_OF_SCOPE: rbu demo CLI (structural index), core wal
