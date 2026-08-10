# CANDIDATES — pcache (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Pluggable pcache2 interface | `sqlite3PcacheInitialize` `src/pcache.c:299`; replaceable via sqlite3_config | Public plugin boundary (sqlite3_pcache_methods2) |
| 002 | Default pcache1 implementation | `pcache1Create` `src/pcache1.c:764`, default install `sqlite3PCacheSetDefault` `:1197` | LRU/purgeable semantics, memory pressure behaviour |
