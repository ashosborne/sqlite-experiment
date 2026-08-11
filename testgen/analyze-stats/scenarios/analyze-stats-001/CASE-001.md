# analyze-stats-001-C001 — run-11 oneshot characterization

Feature: `analyze-stats-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE s1(a); INSERT INTO s1 VALUES(1),(2); CREATE INDEX si ON s1(a); ANALYZE; SELECT count(*) FROM sqlite_master WHERE name='sqlite_stat1'; SELECT stat FROM sqlite_stat1 WHERE idx='si';
```

## Observables

- `row0.col0` = `1`
- `row1.col0` = `2 1`
- `exec.rc` = `0`
- `cb.rows` = `2`
