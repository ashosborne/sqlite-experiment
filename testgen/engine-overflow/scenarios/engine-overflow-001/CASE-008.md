# engine-overflow-001-C008 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE mk(t TEXT); INSERT INTO mk VALUES('qrstuvwxyzqrstuvwxyzqrstuvwxyzqrstuvwxyzqrstuvwxyzqrstuvwxyzqrstuvwxyzqr || [read] SELECT length(t), substr(t, length(t)-16, 15) FROM mk;
```

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `7997`
- `row0.col1` = `OVF900217MARKER`
- `read.rc` = `0`
- `cb.rows` = `1`
