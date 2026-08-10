# Seed — prepare-statement-api
SLICE_ID: prepare-statement-api
SLICE_SEED: "prepared statement lifecycle: prepare/bind/step/column/reset/finalize"
SEED_ENTRYPOINTS: sqlite3_prepare_v2(), sqlite3_step(), sqlite3_bind_*(), sqlite3_column_*(), sqlite3_finalize()
OUT_OF_SCOPE: SQL compilation internals (tokenizer/parser/codegen seeds), one-shot exec wrappers (exec-convenience-api)
Rationale: the core statement state machine — the most characterizable seam in the estate (deterministic inputs → return codes + result columns).
