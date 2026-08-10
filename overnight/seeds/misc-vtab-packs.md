# Seed — misc-vtab-packs
SLICE_ID: misc-vtab-packs
SLICE_SEED: "ext/misc virtual-table extensions (series, csv, zipfile, unionvtab, ...)"
SEED_ENTRYPOINTS: sqlite3_series_init(), sqlite3_csv_init(), sqlite3_zipfile_init()
OUT_OF_SCOPE: vtab mechanism (vtab-core), introspection vtabs in src/
