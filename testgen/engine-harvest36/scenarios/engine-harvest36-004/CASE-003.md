# engine-harvest36-004-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: flags=6 union; NULL passthrough; numeric arguments validate as text (see harness).

## Observables

- `f6_union` = `1,1,0`
- `nulls` = `~,~`
- `numeric_args` = `1,1,1`
