# engine-harvest39-003-C002 — run-11 oneshot characterization

Feature: `engine-harvest39-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: deleting a different row leaves the handle live; deleting its own row expires it (see harness).

## Observables

- `read_after_other_delete_rc` = `0`
- `read_after_own_delete` = `rc=4 bytes=0`
