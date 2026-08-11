# CANDIDATES — vfs-win (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Refines run-1 umbrella feature vfs-os-abstraction-003 (row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Win32 VFS registration + file open | `sqlite3_os_init` `src/os_win.c:5210` (registers `winVfs` `:5211`), `winOpen` decl `:3063` | Windows platform boundary incl. UTF-16 paths, temp files |
| 002 | Windows shared memory + mmap for WAL | `winShmMap` `src/os_win.c:3586` (no-shm stub `:3723`), `winFetch` `:3876` | WAL shm semantics differ from unix — platform-parity risk |
