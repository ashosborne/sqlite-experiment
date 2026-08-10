# Seed — select-codegen
SLICE_ID: select-codegen
SLICE_SEED: "SELECT compilation (joins, subqueries, compound selects)"
SEED_ENTRYPOINTS: sqlite3Select()
OUT_OF_SCOPE: WHERE-loop planning (where-optimizer), window frames (window-functions)
