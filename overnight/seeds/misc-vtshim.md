# Seed — misc-vtshim (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-vtshim
SLICE_SEED: "vtshim wrapper for eponymous-only modules (ext/misc/vtshim.c)"
SEED_ENTRYPOINTS: sqlite3_vtshim_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
