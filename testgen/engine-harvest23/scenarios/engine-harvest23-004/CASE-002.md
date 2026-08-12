# engine-harvest23-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: last_value full frame + default-frame peer behaviour (see harness).

## Observables

- `last_value` = `a,10,30|a,20,30|a,30,30|b,5,15|b,15,15`
- `last_dflt` = `5,5|10,10|15,15|20,20|30,30`
