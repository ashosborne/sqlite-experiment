# engine-harvest37-007-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-007` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stmt_isexplain 0/2 round-trip via sqlite3_stmt_explain (rc 0, bad mode 1); prepared EQP reports 2; EQP column shape (see harness).

## Observables

- `isexplain_plain` = `0`
- `to_eqp_rc` = `0`
- `isexplain_now` = `2`
- `eqp_cols` = `4`
- `back_to_plain_rc` = `0`
- `isexplain_back` = `0`
- `bad_mode_rc` = `1`
- `prepared_eqp_isexplain` = `2`
