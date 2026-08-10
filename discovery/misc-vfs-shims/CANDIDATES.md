# CANDIDATES — misc-vfs-shims (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Cluster of stacking VFS shims (one candidate; per-file evidence). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | VFS shim family | `appendvfs` `ext/misc/appendvfs.c:651` (db appended to another file), `cksumvfs` `ext/misc/cksumvfs.c:833` (page checksums), `vfsstat` `ext/misc/vfsstat.c:806`, `vfstrace` `ext/misc/vfstrace.c`, `vfslog` `ext/misc/vfslog.c`, `tmstmpvfs` `ext/misc/tmstmpvfs.c:1029`, warm-up/trace helpers `mmapwarm.c`/`memtrace.c`/`pcachetrace.c` | Stackable VFS wrappers altering file-level behaviour |
