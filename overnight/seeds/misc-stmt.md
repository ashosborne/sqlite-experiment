# Seed — misc-stmt (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-stmt
SLICE_SEED: "prepared-statement list vtab (ext/misc/stmt.c)"
SEED_ENTRYPOINTS: sqlite3_stmt_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
