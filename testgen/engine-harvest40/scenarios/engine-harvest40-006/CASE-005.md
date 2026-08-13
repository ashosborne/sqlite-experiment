# engine-harvest40-006-C005 — run-11 oneshot characterization

Feature: `engine-harvest40-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: TRUSTED_SCHEMA off refuses non-innocuous app functions inside views (unsafe use), INNOCUOUS functions and direct calls stay allowed (see harness).

## Observables

- `trusted_default` = `42`
- `trusted_off` = `prep.rc=1 err=unsafe use of dbl()`
- `ts_state` = `0`
- `direct_use` = `10`
- `innocuous_off` = `8`
- `trusted_back` = `42`
