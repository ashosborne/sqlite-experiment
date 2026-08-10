# Seed — builtin-scalar-agg-funcs
SLICE_ID: builtin-scalar-agg-funcs
SLICE_SEED: "built-in scalar and aggregate SQL functions registry"
SEED_ENTRYPOINTS: sqlite3RegisterBuiltinFunctions(), aBuiltinFunc[]
OUT_OF_SCOPE: date/time funcs (date-time-funcs), JSON (json-funcs), window funcs, printf/format
Rationale: the SQL-language function surface; registry table is the natural candidate index. NOT one card per function — clustered by family.
