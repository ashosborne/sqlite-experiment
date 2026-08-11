# Seed — misc-completion (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-completion
SLICE_SEED: "completion candidates vtab (ext/misc/completion.c)"
SEED_ENTRYPOINTS: sqlite3_completion_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
