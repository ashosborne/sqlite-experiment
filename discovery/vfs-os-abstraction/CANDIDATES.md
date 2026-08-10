# CANDIDATES — vfs-os-abstraction (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | VFS registration/lookup + OS dispatch | `sqlite3_vfs_register` `src/os.c:408`, `sqlite3_vfs_find` `src/os.c:362`; decl `src/sqlite.h.in:8381` | Public pluggable-platform boundary |
| 002 | Unix VFS (locking styles, shm) | `unixOpen` `src/os_unix.c:6519`, vfs table `:8472`, shm `unixOpenSharedMemory` `:4952` | Default platform implementation incl. POSIX-lock quirks |
| 003 | Windows + kv alternates | `src/os_win.c`, `src/os_kv.c` | Platform variants (win32 API, key-value backend for wasm) |
