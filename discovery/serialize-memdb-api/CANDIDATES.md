# CANDIDATES — serialize-memdb-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | serialize/deserialize round-trip | `src/memdb.c:750` (`sqlite3_serialize`), `src/memdb.c:841` (`sqlite3_deserialize`); decl `src/sqlite.h.in:11248,11326` | Public byte-image API |
| 002 | memdb in-memory VFS | vfs struct `src/memdb.c:137`, `memdbOpen` `src/memdb.c:542` | Pluggable VFS backing deserialize + shared in-memory dbs |
