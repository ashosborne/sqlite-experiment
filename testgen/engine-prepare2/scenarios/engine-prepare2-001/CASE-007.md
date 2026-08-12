# engine-prepare2-001-C007 — run-11 oneshot characterization

Feature: `engine-prepare2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: EXPLAIN QUERY PLAN single-table: 4 columns + SCAN t detail (see harness).

## Observables

- `prepare.rc` = `0`
- `cols` = `4`
- `n0` = `id`
- `n1` = `parent`
- `n2` = `notused`
- `n3` = `detail`
- `detail` = `SCAN t`
