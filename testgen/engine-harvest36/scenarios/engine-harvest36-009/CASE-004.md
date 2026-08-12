# engine-harvest36-009-C004 — run-11 oneshot characterization

Feature: `engine-harvest36-009` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: unconsumed visible-column WHERE stays engine-side; HIDDEN column absent from SELECT * (see harness).

## Observables

- `visible_where` = `2|3`
- `hidden_not_in_star` = `1|2|3|4`
