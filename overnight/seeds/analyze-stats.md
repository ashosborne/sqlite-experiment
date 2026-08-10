# Seed — analyze-stats
SLICE_ID: analyze-stats
SLICE_SEED: "ANALYZE command + sqlite_stat1/stat4 planner statistics"
SEED_ENTRYPOINTS: sqlite3Analyze()
OUT_OF_SCOPE: planner cost model usage (where-optimizer)
