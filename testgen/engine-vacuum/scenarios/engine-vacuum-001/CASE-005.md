# engine-vacuum-001-C005 — run-11 oneshot characterization

Feature: `engine-vacuum-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: churn: page_count shrinks after VACUUM (booleans); survivors (see harness).

## Observables

- `vac` = `rc=0 err=-`
- `shrunk` = `1`
- `after_small` = `1`
- `left` = `5,1,5`
