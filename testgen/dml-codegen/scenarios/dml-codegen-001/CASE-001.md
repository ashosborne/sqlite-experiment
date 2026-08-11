# dml-codegen-001-C001 — run-11 oneshot characterization

Feature: `dml-codegen-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE d(a); INSERT INTO d VALUES(1),(2); UPDATE d SET a=a+10 WHERE a=2; DELETE FROM d WHERE a=1; SELECT a, changes(), total_changes() FROM d;
```

## Observables

- `row0.col0` = `12`
- `row0.col1` = `1`
- `row0.col2` = `4`
- `exec.rc` = `0`
- `cb.rows` = `1`
