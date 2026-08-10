# Seed — json-funcs
SLICE_ID: json-funcs
SLICE_SEED: "JSON SQL functions + json_each/json_tree virtual tables (incl. JSONB)"
SEED_ENTRYPOINTS: sqlite3RegisterJsonFunctions()
OUT_OF_SCOPE: other builtins; vtab mechanism itself (vtab-core)
Rationale: large self-contained surface (33 registry rows) with text and JSONB variants.
