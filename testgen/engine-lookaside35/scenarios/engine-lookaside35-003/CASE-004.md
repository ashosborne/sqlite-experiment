# engine-lookaside35-003-C004 — run-11 oneshot characterization

Feature: `engine-lookaside35-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: HIT current always 0 under live traffic; bad db_status op still ERROR (see harness).

## Observables

- `hit_cur_zero_under_traffic` = `1`
- `badop_rc_still_error` = `1`
