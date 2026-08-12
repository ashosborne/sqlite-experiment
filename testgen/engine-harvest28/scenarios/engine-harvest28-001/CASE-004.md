# engine-harvest28-001-C004 — run-11 oneshot characterization

Feature: `engine-harvest28-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: get_table error: result NULL, errmsg out, rc 1 (see harness).

## Observables

- `rc` = `1`
- `err` = `no such table: nope`
- `res_null` = `1`
