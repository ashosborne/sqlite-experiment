# CANDIDATES — misc-vtab-packs (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Cluster (one candidate; per-file evidence). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Loadable vtab pack (~18 extensions) | `series.c:939` (generate_series), `csv.c:964`, `zipfile.c:2294`, `sqlar.c:109`, `unionvtab.c:1371`, `qpvtab.c:451`, `completion.c:510`, `closure.c:985`, `amatch.c:1512`, `fuzzer.c:1184`, `prefixes.c:310`, `wholenumber.c:276`, `stmt.c:334`, `templatevtab.c:260`, `vtablog.c:712`, `vtshim.c:546`, `btreeinfo.c:439`, `zorder.c:119` (all `ext/misc/`) | Optional table-valued/vtab surfaces |
