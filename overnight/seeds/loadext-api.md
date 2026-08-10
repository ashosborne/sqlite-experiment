# Seed — loadext-api
SLICE_ID: loadext-api
SLICE_SEED: "runtime loadable extension mechanism"
SEED_ENTRYPOINTS: sqlite3_load_extension(), sqlite3_auto_extension()
OUT_OF_SCOPE: individual extensions under ext/ (own seeds)
Rationale: security-sensitive dlopen boundary with enable/disable gate; small file.
