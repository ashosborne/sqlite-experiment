# Seed — misc-regexp (resume run 2, unbundled from misc-func-packs)
SLICE_ID: misc-regexp
SLICE_SEED: "REGEXP operator implementation (ext/misc/regexp.c)"
SEED_ENTRYPOINTS: sqlite3_regexp_init()
OUT_OF_SCOPE: other misc-func-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-func-packs cluster per resume charter.
