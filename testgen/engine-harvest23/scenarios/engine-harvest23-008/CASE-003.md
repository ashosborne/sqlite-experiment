# engine-harvest23-008-C003 — run-11 oneshot characterization

Feature: `engine-harvest23-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: authorizer denies SQLITE_CREATE_TABLE / SQLITE_PRAGMA (see harness).

## Observables

- `crt` = `rc=23 err=not authorized`
- `prag` = `rc=23 err=not authorized`
- `crt_ok` = `rc=0 err=-`
