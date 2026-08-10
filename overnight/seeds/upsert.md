# Seed — upsert
SLICE_ID: upsert
SLICE_SEED: "INSERT ... ON CONFLICT (UPSERT) semantics"
SEED_ENTRYPOINTS: sqlite3UpsertNew(), sqlite3UpsertDoUpdate()
OUT_OF_SCOPE: plain INSERT codegen (dml-codegen), constraint machinery broadly
