# engine-analyze-001-C009 — run-11 oneshot characterization

Feature: `engine-analyze-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: re-ANALYZE replaces rows after INSERT/DELETE churn (see harness).

## Observables

- `before` = `t,ia,2 1`
- `analyze2` = `rc=0 err=-`
- `after` = `t,ia,4 2`
- `rowcount` = `1`
