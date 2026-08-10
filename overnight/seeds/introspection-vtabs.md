# Seed — introspection-vtabs
SLICE_ID: introspection-vtabs
SLICE_SEED: "built-in introspection vtabs: dbstat, sqlite_dbpage, bytecode/tables_used"
SEED_ENTRYPOINTS: sqlite3DbstatRegister(), sqlite3DbpageRegister(), sqlite3VdbeBytecodeVtabInit()
OUT_OF_SCOPE: vtab mechanism (vtab-core)
