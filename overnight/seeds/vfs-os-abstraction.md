# Seed — vfs-os-abstraction
SLICE_ID: vfs-os-abstraction
SLICE_SEED: "VFS layer: registration/find + unix/win/kv implementations"
SEED_ENTRYPOINTS: sqlite3_vfs_register(), sqlite3_vfs_find(), unixOpen()
OUT_OF_SCOPE: memdb VFS (serialize-memdb-api), ext/misc VFS shims (misc-vfs-shims)
