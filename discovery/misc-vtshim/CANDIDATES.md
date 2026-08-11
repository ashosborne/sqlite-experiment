# CANDIDATES — misc-vtshim (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vtab-packs` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | vtshim disposable-module wrapper | `ext/misc/vtshim.c:546` | Wraps a vtab module so it can be safely unregistered while connections exist |
