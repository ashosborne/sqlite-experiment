# engine-harvest28-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest28-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: get_table header + rows cell layout (see harness).

## Observables

- `rc` = `0`
- `nrow` = `2`
- `ncol` = `2`
- `cells` = `a|b|1|x|2|y`
