# engine-overflow-001-C005 — run-11 oneshot characterization

Feature: `engine-overflow-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
[write.rc] CREATE TABLE gr(t TEXT); INSERT INTO gr VALUES('short'); || [grow.rc] UPDATE gr SET t='abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabc || [read] SELECT length(t), substr(t,1,5) FROM gr;
```

## Observables

- `write.rc` = `0`
- `grow.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `6000`
- `row0.col1` = `abcde`
- `read.rc` = `0`
- `cb.rows` = `1`
