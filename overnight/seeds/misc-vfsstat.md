# Seed — misc-vfsstat (resume run 2, unbundled from misc-vfs-shims)
SLICE_ID: misc-vfsstat
SLICE_SEED: "I/O counters VFS (ext/misc/vfsstat.c)"
SEED_ENTRYPOINTS: sqlite3_vfsstat_init()
OUT_OF_SCOPE: other misc-vfs-shims members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vfs-shims cluster per resume charter.
