# Seed — misc-vtablog (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-vtablog
SLICE_SEED: "vtablog tracing vtab (ext/misc/vtablog.c)"
SEED_ENTRYPOINTS: sqlite3_vtablog_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
