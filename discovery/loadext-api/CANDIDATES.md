# CANDIDATES — loadext-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | load_extension + enable gate | `src/loadext.c:731` public, core `:561`, gate `:762`; decl `src/sqlite.h.in:7609` | dlopen/dlsym boundary + entrypoint-name derivation |
| 002 | Auto-extension registry | `src/loadext.c:811,861,889`, applied at open via `sqlite3AutoLoadExtensions` `:911` | Process-global registry run on every new connection |
