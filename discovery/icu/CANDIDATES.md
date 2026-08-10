# CANDIDATES — icu (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | ICU function/collation pack | `sqlite3IcuInit` `ext/icu/icu.c:538` + scalar table `:546` (`icu_load_collation` `:547`, `icuLikeFunc` `:213`, `icuRegexpFunc` `:280`, `icuLoadCollation` `:465`) | Replaces LIKE/upper/lower with ICU-aware versions; runtime collation loading |
