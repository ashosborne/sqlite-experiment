# CANDIDATES — json-funcs (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Registry `sqlite3RegisterJsonFunctions` `src/json.c:5660` (33 JFUNCTION rows). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Extraction (json_extract, -> / ->>, jsonb_extract) | `jsonExtractFunc` `src/json.c:4071` | Path-expression read surface |
| 002 | Mutation (json_set/insert/replace/patch/remove + jsonb_*) | `jsonSetFunc` `src/json.c:4548`, `jsonPatchFunc` `src/json.c:4389`, registry rows `:5673-5696` | Write surface incl. RFC-7396 patch |
| 003 | Validation/typing (json_valid, json_type, json_error_position) | `jsonValidFunc` `src/json.c:4700` | Well-formedness contract incl. JSONB flavors |
| 004 | json_each / json_tree table-valued functions | `jsonEachModule` `src/json.c:5627` | Enumeration vtabs used in joins |
