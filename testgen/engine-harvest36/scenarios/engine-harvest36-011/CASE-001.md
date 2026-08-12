# engine-harvest36-011-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-011` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: blob_open on an attached schema reads real bytes; unknown db errors no such table: db.t (see harness).

## Observables

- `aux_open_rc` = `0`
- `aux_bytes` = `5`
- `aux_read` = `2,3`
- `baddb_rc` = `1 err=no such table: nosuchdb.t`
