# engine-conn-001-C004 — run-11 oneshot characterization

Feature: `engine-conn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: close with open transaction rolls back (file reopen proves it) (see harness).

## Observables

- `close` = `0`
- `reopen` = `1,1`
