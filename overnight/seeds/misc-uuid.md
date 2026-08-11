# Seed — misc-uuid (resume run 2, unbundled from misc-func-packs)
SLICE_ID: misc-uuid
SLICE_SEED: "uuid(), uuid_str(), uuid_blob() (ext/misc/uuid.c)"
SEED_ENTRYPOINTS: sqlite3_uuid_init()
OUT_OF_SCOPE: other misc-func-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-func-packs cluster per resume charter.
