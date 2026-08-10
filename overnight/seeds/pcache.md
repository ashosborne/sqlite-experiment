# Seed — pcache
SLICE_ID: pcache
SLICE_SEED: "pluggable page cache (pcache interface + default pcache1)"
SEED_ENTRYPOINTS: sqlite3_config(SQLITE_CONFIG_PCACHE2), sqlite3PCacheSetDefault()
OUT_OF_SCOPE: pager usage of the cache (pager)
