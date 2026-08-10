# Seed — misc-vfs-shims
SLICE_ID: misc-vfs-shims
SLICE_SEED: "ext/misc VFS wrapper extensions (append, checksum, tracing, stats)"
SEED_ENTRYPOINTS: sqlite3_appendvfs_init(), sqlite3_cksumvfs_init(), sqlite3_vfsstat_init()
OUT_OF_SCOPE: core VFS (vfs-os-abstraction)
