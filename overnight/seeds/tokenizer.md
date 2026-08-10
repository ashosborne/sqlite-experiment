# Seed — tokenizer
SLICE_ID: tokenizer
SLICE_SEED: "SQL tokenizer + statement-completeness checker"
SEED_ENTRYPOINTS: sqlite3RunParser(), sqlite3GetToken(), sqlite3_complete()
OUT_OF_SCOPE: grammar reductions (parser-grammar)
