# Seed — misc-percentile (resume run 2, unbundled from misc-func-packs)
SLICE_ID: misc-percentile
SLICE_SEED: "percentile(Y,P), median() (ext/misc/percentile.c)"
SEED_ENTRYPOINTS: sqlite3_percentile_init()
OUT_OF_SCOPE: other misc-func-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-func-packs cluster per resume charter.
