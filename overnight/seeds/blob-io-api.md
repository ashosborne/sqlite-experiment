# Seed — blob-io-api
SLICE_ID: blob-io-api
SLICE_SEED: "incremental blob I/O handles (sqlite3_blob_*)"
SEED_ENTRYPOINTS: sqlite3_blob_open(), sqlite3_blob_read(), sqlite3_blob_write()
OUT_OF_SCOPE: column blob accessors (prepare-statement-api), zeroblob binding
Rationale: self-contained handle API over a single table cell; crisp read/write/expiry semantics.
