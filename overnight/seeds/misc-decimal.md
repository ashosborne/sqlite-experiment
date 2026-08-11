# Seed — misc-decimal (resume run 2, unbundled from misc-func-packs)
SLICE_ID: misc-decimal
SLICE_SEED: "decimal_add/sub/mul/cmp + collation (ext/misc/decimal.c)"
SEED_ENTRYPOINTS: sqlite3_decimal_init()
OUT_OF_SCOPE: other misc-func-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-func-packs cluster per resume charter.
