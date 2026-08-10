# Seed — vtab-core
SLICE_ID: vtab-core
SLICE_SEED: "virtual table mechanism (module registry, xCreate/xConnect, xBestIndex plumbing)"
SEED_ENTRYPOINTS: sqlite3_create_module(), sqlite3_declare_vtab()
OUT_OF_SCOPE: individual vtab implementations (json_each, dbstat, ext/*)
