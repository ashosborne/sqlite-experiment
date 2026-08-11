# Seed — misc-btreeinfo (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-btreeinfo
SLICE_SEED: "btree metadata vtab (ext/misc/btreeinfo.c)"
SEED_ENTRYPOINTS: sqlite3_btreeinfo_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
