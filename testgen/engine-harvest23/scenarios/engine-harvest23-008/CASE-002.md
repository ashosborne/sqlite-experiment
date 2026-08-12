# engine-harvest23-008-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: authorizer denies SQLITE_UPDATE / SQLITE_DELETE (see harness).

## Observables

- `upd` = `rc=23 err=not authorized`
- `del` = `rc=23 err=not authorized`
- `row` = `1`
