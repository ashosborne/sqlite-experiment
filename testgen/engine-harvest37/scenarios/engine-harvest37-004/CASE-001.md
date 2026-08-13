# engine-harvest37-004-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ROWS frame EXCLUDE CURRENT ROW / NO OTHERS / GROUP / TIES change frame membership (see harness).

## Observables

- `excl_current` = `1,20|2,35|2,50|3,65|4,30`
- `excl_none` = `1,30|2,55|2,75|3,95|4,70`
- `excl_group` = `1,20|2,10|2,30|3,65|4,30`
- `excl_ties` = `1,30|2,30|2,55|3,95|4,70`
