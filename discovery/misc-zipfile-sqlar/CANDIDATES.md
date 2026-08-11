# CANDIDATES — misc-zipfile-sqlar (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vtab-packs` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | zipfile vtab + sqlar functions (one optional pack) | `ext/misc/zipfile.c:2294`, `ext/misc/sqlar.c:109` | Read/write zip archives as tables; sqlar_compress/uncompress build the SQLite-archive format on it (genuinely one pack, kept clustered) |
