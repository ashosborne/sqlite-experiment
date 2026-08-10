# CANDIDATES — util-primitives (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Cross-cutting primitives (UTF-8/16 codecs, PRNG incl. public sqlite3_randomness, symbol hash) | `sqlite3Utf8Read` `src/utf.c:175`, `Utf8CharLen` `:475`, `sqlite3_randomness` `src/random.c:59`, `sqlite3HashInit` `src/hash.c:23` | Shared behaviour every surface depends on; PRNG is a public API |

bitvec.c / rowset.c noted as purely internal helpers — skipped-with-reason (no callable/SQL-visible boundary).
