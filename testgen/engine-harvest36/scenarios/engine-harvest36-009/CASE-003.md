# engine-harvest36-009-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-009` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: EQ on HIDDEN lim offered via xBestIndex, consumed, value reaches xFilter argv (rows bounded) (see harness).

## Observables

- `pushdown` = `1|2|3|4|5`
- `constraint_offered` = `1`
- `filter_argc` = `1`
- `pushdown_count` = `4`
- `offered_again` = `1`
