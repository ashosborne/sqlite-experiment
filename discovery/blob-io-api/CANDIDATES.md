# CANDIDATES — blob-io-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Blob handle open/close/reopen | `src/vdbeblob.c:121,360,503`; decl `src/sqlite.h.in:8206` | Handle constructor over one row/column |
| 002 | Incremental read/write + bounds | `src/vdbeblob.c:471,478` via shared `blobReadWrite` `src/vdbeblob.c:381`, size `:488` | Offset/length contract, expired-handle errors |
