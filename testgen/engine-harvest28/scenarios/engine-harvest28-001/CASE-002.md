# engine-harvest28-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest28-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: get_table NULL cells come back as NULL pointers (see harness).

## Observables

- `nrow` = `2`
- `ncol` = `2`
- `r1b_null` = `1`
- `r2a_null` = `1`
- `r1a` = `1`
- `r2b` = `z`
