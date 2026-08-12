# engine-checkupd-002-C005 — run-11 oneshot characterization

Feature: `engine-checkupd-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: plain UPDATE (ABORT) undoes the whole statement (1,9) (see harness).

## Observables

- `upd.rc` = `19`
- `a` = `1`
- `a` = `9`
