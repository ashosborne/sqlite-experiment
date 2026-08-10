# CANDIDATES — misc-func-packs (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Cluster (one candidate; per-file evidence). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Loadable SQL-function pack (~18 extensions) | encoding: `base64.c:278`, `base85.c:355`, `basexx.c:70`; crypto: `sha1.c:418`, `shathree.c:830`; numeric: `decimal.c:922`, `ieee754.c:329`, `percentile.c:480`, `totype.c:511`, `uint.c:84`; text: `regexp.c:901`, `spellfix.c:3085`, `nextchar.c:296`, `rot13.c:100`; misc: `uuid.c:213`, `fossildelta.c:1111`, `compress.c:122`, `urifuncs.c:180` (all `ext/misc/`) | Optional SQL-visible function surfaces loaded per-connection |
