# Seed — misc-appendvfs (resume run 2, unbundled from misc-vfs-shims)
SLICE_ID: misc-appendvfs
SLICE_SEED: "apndvfs shim (ext/misc/appendvfs.c)"
SEED_ENTRYPOINTS: sqlite3_appendvfs_init()
OUT_OF_SCOPE: other misc-vfs-shims members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vfs-shims cluster per resume charter.
