# engine-harvest42-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest42-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DENY of code 33 fails prepare rc 23 not authorized, stopping after the 33 consult (see harness).

## Observables

- `a_deny` = `prep.rc=23 err=not authorized LOG=[21|~|~|~|~][21|~|~|~|c][33|~|~|~|c]`
