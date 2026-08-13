# engine-harvest40-005-C003 — run-11 oneshot characterization

Feature: `engine-harvest40-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: second path argument roots the walk; scalar roots yield one row; row counts (see harness).

## Observables

- `each_second` = `a,1`
- `tree_second` = `$.a,{"b":4}|$.a.b,4`
- `each_scalar` = `~,9,integer,0`
- `tree_count` = `6`
- `each_count` = `2`
