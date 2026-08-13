# engine-harvest40-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: legacy sqlite3_trace fires per statement; exec text keeps the terminator, prepared text has bound parameters EXPANDED (see harness).

## Observables

- `nlt` = `2`
- `log` = `[T:INSERT INTO t VALUES(1);][T:SELECT a FROM t WHERE a = 1]`
