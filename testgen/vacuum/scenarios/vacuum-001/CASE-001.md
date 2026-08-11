# vacuum-001-C001 — run-11 oneshot characterization

Feature: `vacuum-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE v1(a); INSERT INTO v1 VALUES(zeroblob(1000)); DROP TABLE v1; VACUUM; SELECT 1;
```

## Observables

- `row0.col0` = `1`
- `exec.rc` = `0`
- `cb.rows` = `1`
