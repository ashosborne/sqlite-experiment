# Seed — misc-cksumvfs (resume run 2, unbundled from misc-vfs-shims)
SLICE_ID: misc-cksumvfs
SLICE_SEED: "checksum VFS shim (ext/misc/cksumvfs.c)"
SEED_ENTRYPOINTS: sqlite3_cksumvfs_init()
OUT_OF_SCOPE: other misc-vfs-shims members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vfs-shims cluster per resume charter.
