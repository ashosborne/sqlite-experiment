# engine-harvest41-005-C001 — run-11 oneshot characterization

Feature: `engine-harvest41-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DROP TABLE fires code 11 (s1 = table, s3 = main) - outer only; DENY rc 23 leaves the table (see harness).

## Observables

- `drop_t` = `rc=0 err=- LOG=[11|t|~|main|~]`
- `drop_deny` = `rc=23 err=not authorized LOG=[11|t2|~|main|~]`
- `t2_survives` = `rows=t2 LOG=`
