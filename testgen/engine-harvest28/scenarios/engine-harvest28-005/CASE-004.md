# engine-harvest28-005-C004 — run-11 oneshot characterization

Feature: `engine-harvest28-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: percentile out-of-range fraction errors (see harness).

## Observables

- `bad` = `rc=1 err=the fraction argument to percentile() is not between 0.0 and 100.0`
- `neg` = `rc=1 err=the fraction argument to percentile() is not between 0.0 and 100.0`
