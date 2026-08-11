# CANDIDATES — misc-basexx (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-func-packs` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | base64 + base85 + combined basexx encoders (one optional pack) | `ext/misc/basexx.c:70`, `ext/misc/base64.c:278`, `ext/misc/base85.c:355` | base64()/base85() blob<->text conversions; basexx.c registers both — kept as the one genuine pack |
