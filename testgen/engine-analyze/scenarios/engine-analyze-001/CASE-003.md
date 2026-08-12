# engine-analyze-001-C003 — run-11 oneshot characterization

Feature: `engine-analyze-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: one index: N M selectivity (3 rows, 2 distinct -> 3 2) (see harness).

## Observables

- `analyze` = `rc=0 err=-`
- `stat1` = `t,ib,3 2`
