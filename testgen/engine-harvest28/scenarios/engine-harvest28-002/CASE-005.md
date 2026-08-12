# engine-harvest28-002-C005 — run-11 oneshot characterization

Feature: `engine-harvest28-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: db_status LOOKASIDE_USED shape; invalid op -> ERROR(1) (see harness).

## Observables

- `rc` = `0`
- `cur_le_hi` = `1`
- `cur_nonneg` = `1`
- `badop_rc` = `1`
