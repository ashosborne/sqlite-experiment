# engine-none29-001-C006 — run-11 oneshot characterization

Feature: `engine-none29-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: multi-statement body with one qualified DML -> whole CREATE rejected (see harness).

## Observables

- `multi` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
- `not_made` = `0`
