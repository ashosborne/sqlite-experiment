# engine-setops-001-C004 — run-11 oneshot characterization

Feature: `engine-setops-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE s3(a); INSERT INTO s3 VALUES(5); SELECT a FROM s3 EXCEPT SELECT 5;
```

## Observables

- `exec.rc` = `0`
- `cb.rows` = `0`
