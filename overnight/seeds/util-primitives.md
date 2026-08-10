# Seed — util-primitives
SLICE_ID: util-primitives
SLICE_SEED: "shared primitives: UTF conversion, PRNG, hash tables"
SEED_ENTRYPOINTS: sqlite3Utf8Read(), sqlite3_randomness(), sqlite3HashInit()
OUT_OF_SCOPE: bitvec/rowset (internal-only helpers, noted), value semantics (vdbe-engine)
