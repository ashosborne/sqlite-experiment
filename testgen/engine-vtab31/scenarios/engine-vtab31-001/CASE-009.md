# engine-vtab31-001-C009 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: CREATE VIRTUAL TABLE without args -> module default (see harness).

## Observables

- `noargs` = `rc=0 err=-`
- `dflt` = `1|2|3`
