# engine-join-001-C007 — run-11 oneshot characterization

Feature: `engine-join-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

One `sqlite3_exec` call on a fresh `:memory:` db:

```sql
CREATE TABLE ka(k INTEGER); INSERT INTO ka VALUES(910033); CREATE TABLE kb(k INTEGER, t TEXT); INSERT INTO kb VALUES(910033,'hit'),(7,'miss'); SELECT ka.k, kb.t FROM ka JOIN kb ON ka.k = kb.k;
```

## Observables

- `row0.col0` = `910033`
- `row0.col1` = `hit`
- `exec.rc` = `0`
- `cb.rows` = `1`
