# Seed — exec-convenience-api
SLICE_ID: exec-convenience-api
SLICE_SEED: "one-shot SQL execution convenience wrappers (exec, get_table)"
SEED_ENTRYPOINTS: sqlite3_exec(), sqlite3_get_table()
OUT_OF_SCOPE: prepared-statement core (prepare-statement-api)
Rationale: thin legacy wrapper seam; small and independently characterizable (callback ordering, error message ownership).
