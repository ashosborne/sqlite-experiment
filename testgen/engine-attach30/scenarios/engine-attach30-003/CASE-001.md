# engine-attach30-003-C001 — run-11 oneshot characterization

Feature: `engine-attach30-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: aux trigger cross-db DML rejected; aux view referencing main rejected (see harness).

## Observables

- `xtrig` = `rc=1 err=qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers`
- `xview` = `rc=1 err=view vv cannot reference objects in database main`
