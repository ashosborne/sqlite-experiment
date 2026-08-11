# pragma-surface-002-C004 — run-11 oneshot characterization

Feature: `pragma-surface-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
SELECT count(*) FROM pragma_function_list, (SELECT 1) WHERE (SELECT count(*) FROM pragma_module_list)=18 AND (SELECT count(*) FROM pragma_pragma_list)=66;
```

## Observables

- `row0.col0` = `0`
- `exec.rc` = `0`
- `cb.rows` = `1`
