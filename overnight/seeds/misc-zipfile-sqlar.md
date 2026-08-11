# Seed — misc-zipfile-sqlar (resume run 2, unbundled from misc-vtab-packs)
SLICE_ID: misc-zipfile-sqlar
SLICE_SEED: "zip archive vtab + sqlar compress/uncompress (ext/misc/zipfile.c + sqlar.c)"
SEED_ENTRYPOINTS: sqlite3_zipfile_init()
OUT_OF_SCOPE: other misc-vtab-packs members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vtab-packs cluster per resume charter.
