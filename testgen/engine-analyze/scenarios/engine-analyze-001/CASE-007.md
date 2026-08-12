# engine-analyze-001-C007 — run-11 oneshot characterization

Feature: `engine-analyze-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ANALYZE <index> writes only that index row (see harness).

## Observables

- `analyze` = `rc=0 err=-`
- `stat1` = `t,ia,2 1`
