# json-funcs-002-C001 — run-11 oneshot characterization

Feature: `json-funcs-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT json_set('{}','$.a',1), json_remove('{"a":1,"b":2}','$.b'), json_patch('{"a":1}','{"b":2}');
```

## Observables

- `row0.col0` = `{"a":1}`
- `row0.col1` = `{"a":1}`
- `row0.col2` = `{"a":1,"b":2}`
- `exec.rc` = `0`
- `cb.rows` = `1`
