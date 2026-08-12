# engine-harvest23-006-C003 — run-11 oneshot characterization

Feature: `engine-harvest23-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: extended FK errcode 787 (see harness).

## Observables

- `fk` = `rc=19 err=FOREIGN KEY constraint failed`
- `fk_ext` = `787`
