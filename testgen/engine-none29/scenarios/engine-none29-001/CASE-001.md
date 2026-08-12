# engine-none29-001-C001 — run-11 oneshot characterization

Feature: `engine-none29-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: qualified INSERT in trigger body -> rejected; trigger not created (see harness).

## Observables

- `qual_insert` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
- `not_made` = `0`
