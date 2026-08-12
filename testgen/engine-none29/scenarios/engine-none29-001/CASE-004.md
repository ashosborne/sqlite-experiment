# engine-none29-001-C004 — run-11 oneshot characterization

Feature: `engine-none29-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: qualified table in a SELECT inside the trigger body is allowed (see harness).

## Observables

- `qual_select_ok` = `rc=0 err=-`
- `fire` = `rc=0 err=-`
- `cnt` = `1`
