# engine-none29-001-C002 — run-11 oneshot characterization

Feature: `engine-none29-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: qualified UPDATE and DELETE in trigger body -> rejected (see harness).

## Observables

- `qual_update` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
- `qual_delete` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
