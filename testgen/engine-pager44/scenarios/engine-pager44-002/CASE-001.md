# engine-pager44-002-C001 — run-11 oneshot characterization

Feature: `engine-pager44-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ROLLBACK restores the pre-images: UPDATE, DELETE, and a multi-row UPDATE all revert (see harness).

## Observables

- `upd_rollback` = `one`
- `del_rollback` = `two`
- `multi_rollback` = `one|two|three|four`
