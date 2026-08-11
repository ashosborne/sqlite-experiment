# Seed — misc-fuzzer (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-fuzzer
SLICE_SEED: "fuzzer vtab (ext/misc/fuzzer.c)"
SEED_ENTRYPOINTS: sqlite3_fuzzer_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
