# Seed — attach-detach
SLICE_ID: attach-detach
SLICE_SEED: "ATTACH / DETACH DATABASE statement behaviour"
SEED_ENTRYPOINTS: sqlite3Attach(), sqlite3Detach(), attachFunc()
OUT_OF_SCOPE: schema loading internals (ddl-schema), open flags (connection-lifecycle-api)
Rationale: SQL-language surface with clear observables (database list, cross-db name resolution limits).
