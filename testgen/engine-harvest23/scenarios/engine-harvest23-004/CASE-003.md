# engine-harvest23-004-C003 — run-11 oneshot characterization

Feature: `engine-harvest23-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: nth_value(2) + out-of-range NULL (see harness).

## Observables

- `nth_2` = `a,10,20|a,20,20|a,30,20|b,5,15|b,15,15`
- `nth_oor` = `5,~|10,~|15,~|20,~|30,~`
