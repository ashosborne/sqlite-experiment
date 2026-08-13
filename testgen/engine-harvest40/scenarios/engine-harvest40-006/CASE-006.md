# engine-harvest40-006-C006 — run-11 oneshot characterization

Feature: `engine-harvest40-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: load_extension gates: C API refused not authorized until DBCONFIG 1005; the SQL function stays refused under 1005 (C's API/SQL split) (see harness).

## Observables

- `capi_disabled` = `rc=1 err=not authorized`
- `sql_disabled` = `rc=1 err=not authorized`
- `capi_state` = `1`
- `sql_capi_on` = `rc=1 err=not authorized`
