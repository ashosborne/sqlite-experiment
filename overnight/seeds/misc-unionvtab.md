# Seed — misc-unionvtab (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-unionvtab
SLICE_SEED: "union of identical tables across dbs (ext/misc/unionvtab.c)"
SEED_ENTRYPOINTS: sqlite3_unionvtab_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
