# engine-kitchen-001-C003 — run-11 oneshot characterization

Feature: `engine-kitchen-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE k(a INTEGER); INSERT INTO k VALUES(10); UPDATE k SET a=11; SELECT a FROM k;
```

## Observables

- `row0.col0` = `11`
- `exec.rc` = `0`
- `cb.rows` = `1`
