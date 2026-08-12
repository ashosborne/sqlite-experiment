# engine-vtab31-003-C007 — run-11 oneshot characterization

Feature: `engine-vtab31-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WHERE on HIDDEN column (checked via xColumn, not pushdown) (see harness).

## Observables

- `hidden_where` = `1|2|3|4|5`
- `hidden_where_miss` = ``
