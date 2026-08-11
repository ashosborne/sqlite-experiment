# Seed — misc-series (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-series
SLICE_SEED: "generate_series TVF (ext/misc/series.c)"
SEED_ENTRYPOINTS: sqlite3_series_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
