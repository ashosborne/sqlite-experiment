# CANDIDATES — misc-fuzzer (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vtab-packs` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | fuzzer vtab (string mutation search) | `ext/misc/fuzzer.c:1184` | Generates cost-ranked string mutations from a rules table (typo-correction searches) |
