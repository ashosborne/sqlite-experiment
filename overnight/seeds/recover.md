# Seed — recover
SLICE_ID: recover
SLICE_SEED: "corrupt-database recovery API"
SEED_ENTRYPOINTS: sqlite3_recover_init(), sqlite3_recover_step()
OUT_OF_SCOPE: dbdata vtab internals (cited as dependency), test_recover.c harness (skipped-with-reason)
