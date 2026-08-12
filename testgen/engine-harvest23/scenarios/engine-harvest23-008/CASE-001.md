# engine-harvest23-008-C001 — run-11 oneshot characterization

Feature: `engine-harvest23-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: authorizer denies SQLITE_INSERT (rc 23) (see harness).

## Observables

- `ins` = `rc=23 err=not authorized`
- `ins_ok` = `rc=0 err=-`
- `cnt` = `1`
