# CANDIDATES — vtab-core (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Module registration + vtab lifecycle | `sqlite3_create_module` `src/vtab.c:108`, `_v2` `:123`, `VtabCallCreate` `:774`, `VtabCallConnect` `:699`; decl `src/sqlite.h.in:8002` | Public extension boundary for table implementations |
| 002 | declare_vtab + vtab_config contract | `sqlite3_declare_vtab` `src/vtab.c:815`, `sqlite3_vtab_config` `:1339` | Schema declaration + constraint-support negotiation |
