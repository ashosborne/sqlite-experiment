# CANDIDATES — prepare-statement-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Prepare family (UTF-8/16, v1/v2/v3, prep flags) | `src/prepare.c:943,955,973,1071,1083,1095`; core `sqlite3Prepare` `src/prepare.c:700`; retry wrapper `sqlite3LockAndPrepare` `src/prepare.c:854`; decl `src/sqlite.h.in:4637` | Statement constructor |
| 002 | Step execution + return-code contract | `src/vdbeapi.c:980` (`sqlite3_step`); decl `src/sqlite.h.in:5347` | ROW/DONE/BUSY/error state machine |
| 003 | Parameter binding | `src/vdbeapi.c:1816,1838,1849,1852,1863,1892,1927,1961` (`bind_blob..zeroblob`), `src/vdbeapi.c:155` (`clear_bindings`) | Typed input contract |
| 004 | Column result access | `src/vdbeapi.c:1448,1458,1468,1473,1478`, count `src/vdbeapi.c:1333` | Typed output contract incl. type coercions |
| 005 | Reset / finalize / auto-reprepare | `src/vdbeapi.c:134,105`; `sqlite3Reprepare` `src/prepare.c:904` | Statement teardown + schema-change retry |
| 006 | Statement introspection | `src/vdbeapi.c:2090` (`stmt_readonly`), `:2141` (`stmt_busy`), `:2105` (`stmt_explain`) | Read-only observability API |
