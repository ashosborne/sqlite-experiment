# Seed — misc-tmstmpvfs (resume run 2, unbundled from misc-vfs-shims)
SLICE_ID: misc-tmstmpvfs
SLICE_SEED: "tmstmp VFS shim (ext/misc/tmstmpvfs.c)"
SEED_ENTRYPOINTS: sqlite3_tmstmpvfs_init()
OUT_OF_SCOPE: other misc-vfs-shims members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vfs-shims cluster per resume charter.
