# engine-none29-001-C005 — run-11 oneshot characterization

Feature: `engine-none29-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: BEFORE trigger with qualified DML -> rejected (see harness).

## Observables

- `before_qual` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
