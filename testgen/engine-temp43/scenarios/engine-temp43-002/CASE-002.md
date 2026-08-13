# engine-temp43-002-C002 — run-11 oneshot characterization

Feature: `engine-temp43-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DENY of 4/13 is rc 23 and the object survives (see harness).

## Observables

- `a_deny_ct` = `rc=23 err=not authorized LOG=[4|at3|~|temp|~]`
- `a_deny_drop` = `rc=23 err=not authorized LOG=[13|at2|~|temp|~]`
- `at2_survives` = `rows=1`
