# CANDIDATES — backup-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Backup lifecycle init/step/finish | `src/backup.c:141,325,598`; decl `src/sqlite.h.in:9822` | Callable boundary; page-batched copy loop |
| 002 | Progress introspection | `src/backup.c:654` (`remaining`), `src/backup.c:668` (`pagecount`) | Observable progress contract |
| 003 | Source-write coordination during backup | `sqlite3BackupUpdate` `src/backup.c:715`, `sqlite3BackupRestart` `src/backup.c:730` | Internal seam: concurrent source writes restart/patch the copy |
