# Seed — dml-codegen
SLICE_ID: dml-codegen
SLICE_SEED: "INSERT/UPDATE/DELETE compilation incl. constraint checks"
SEED_ENTRYPOINTS: sqlite3Insert(), sqlite3Update(), sqlite3DeleteFrom()
OUT_OF_SCOPE: UPSERT (upsert), FK enforcement (foreign-keys), triggers
