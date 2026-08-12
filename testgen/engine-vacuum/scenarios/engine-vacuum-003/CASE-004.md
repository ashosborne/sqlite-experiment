# engine-vacuum-003-C004 — run-11 oneshot characterization

Feature: `engine-vacuum-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: INTO inside a transaction -> C error (see harness).

## Observables

- `into` = `rc=1 err=cannot VACUUM from within a transaction`
- `commit` = `rc=0 err=-`
