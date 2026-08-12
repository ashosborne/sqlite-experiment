# engine-harvest28-001-C006 — run-11 oneshot characterization

Feature: `engine-harvest28-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: get_table over an expression query; free_table(NULL) tolerated (see harness).

## Observables

- `rc` = `0`
- `nrow` = `1`
- `ncol` = `2`
- `cells` = `s|'ok'|5|ok`
