# Seed — misc-vfslog (resume run 2, unbundled from misc-vfs-shims)
SLICE_ID: misc-vfslog
SLICE_SEED: "VFS op log to file (ext/misc/vfslog.c)"
SEED_ENTRYPOINTS: sqlite3_register_vfslog()
OUT_OF_SCOPE: other misc-vfs-shims members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vfs-shims cluster per resume charter.
