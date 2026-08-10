# Seed — malloc-subsystem
SLICE_ID: malloc-subsystem
SLICE_SEED: "memory allocation: public malloc API, lookaside, alternative allocators"
SEED_ENTRYPOINTS: sqlite3_malloc64(), sqlite3Malloc()
OUT_OF_SCOPE: status counters (error-status-api), pcache memory
