# engine-attach30-003-C002 — run-11 oneshot characterization

Feature: `engine-attach30-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: aux trigger with qualified same-schema DML rejected (see harness).

## Observables

- `qual_same` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
