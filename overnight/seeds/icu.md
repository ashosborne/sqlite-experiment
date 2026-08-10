# Seed — icu
SLICE_ID: icu
SLICE_SEED: "ICU extension: unicode-aware LIKE/REGEXP/upper/lower + collations"
SEED_ENTRYPOINTS: sqlite3IcuInit()
OUT_OF_SCOPE: core LIKE machinery (builtin-scalar-agg-funcs)
