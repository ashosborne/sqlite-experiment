# engine-checkupd-002-C004 — run-11 oneshot characterization

Feature: `engine-checkupd-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: UPDATE OR FAIL keeps earlier row changes of the same statement (6,9) (see harness).

## Observables

- `upd.rc` = `19`
- `a` = `6`
- `a` = `9`
