# engine-harvest36-006-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: VACUUM INTO URI filenames are literal on this pin (USE_URI off): unable to open database (see harness).

## Observables

- `uri_into` = `rc=14 err=unable to open database: file:/tmp/h36u.db`
- `no_uri_file` = `1`
- `uri_into_params` = `rc=14 err=unable to open database: file:/tmp/h36u2.db?mode=rwc`
