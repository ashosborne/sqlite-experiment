# CANDIDATES — vfs-unix-variants (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. New entrypoints not covered by run-1 vfs-os-abstraction-002. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Locking-style matrix (posix/dotlock/flock/sem/afp/nfs/proxy/none) | `SQLITE_ENABLE_LOCKING_STYLE` gate `src/os_unix.c:66`, style docs `:61,119`, apple gate `:143` | 8 lock strategies selected per-filesystem at open — behaviour differs on NFS/AFP |
| 002 | Proxy locking (conch file) | `proxyIoMethods` `src/os_unix.c:5928`, `proxyLock` decl `:5923`, method switch `:8207,8231` | macOS conch-file protocol replacing byte-range locks |
| 003 | VxWorks named-semaphore paths | `OS_VXWORKS` sections `src/os_unix.c:137` | Embedded-RTOS variant with different lock + path semantics |
