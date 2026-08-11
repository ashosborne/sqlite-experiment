# Seed — misc-vfstrace (resume run 2, unbundled from misc-vfs-shims)
SLICE_ID: misc-vfstrace
SLICE_SEED: "VFS call tracer (ext/misc/vfstrace.c)"
SEED_ENTRYPOINTS: vfstrace_register()
OUT_OF_SCOPE: other misc-vfs-shims members; run-1 umbrella row (untouched)
Rationale: thin unbundle of the run-1 misc-vfs-shims cluster per resume charter.
