# json-funcs-001-C001 — run-11 oneshot characterization

Feature: `json-funcs-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT json_extract('{"a":{"b":2}}','$.a.b'), '{"a":1}' -> '$.a', '{"a":1}' ->> '$.a';
```

## Observables

- `row0.col0` = `2`
- `row0.col1` = `1`
- `row0.col2` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
