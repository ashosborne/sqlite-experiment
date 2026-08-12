# engine-pragma34-001-C001 — run-11 oneshot characterization

Feature: `engine-pragma34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: user_version / application_id get-set round-trips (see harness).

## Observables

- `uv0` = `0`
- `uv_set` = `rc=0 err=-`
- `uv1` = `42`
- `app0` = `0`
- `app_set` = `rc=0 err=-`
- `app1` = `1234`
