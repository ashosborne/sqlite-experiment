# CANDIDATES — fts5 (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind. ext/fts5/tool/ generator scripts skipped-with-reason.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | FTS5 vtab module (create/query lifecycle) | init `ext/fts5/fts5_main.c:3891`, module wiring `fts5Init` `:3763`, `fts5CreateMethod` `:477` | Primary FTS surface: CREATE VIRTUAL TABLE ... USING fts5 + MATCH queries |
| 002 | Tokenizer API (v1/v2) + built-in tokenizers | registration path `ext/fts5/fts5_main.c:3217,3252`, tokenizer module structs `ext/fts5/fts5_tokenize.c:556,587` | Pluggable analysis chain (unicode61, ascii, porter, trigram) |
| 003 | Auxiliary functions (highlight, snippet, bm25) + fts5_api | user-visible api object `ext/fts5/fts5_main.c:79` | Ranking/snippet surface + C-level extension API |
