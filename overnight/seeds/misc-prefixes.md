# Seed — misc-prefixes (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-prefixes
SLICE_SEED: "prefixes TVF (ext/misc/prefixes.c)"
SEED_ENTRYPOINTS: sqlite3_prefixes_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
