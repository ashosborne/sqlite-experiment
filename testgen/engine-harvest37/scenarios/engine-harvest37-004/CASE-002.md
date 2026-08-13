# engine-harvest37-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest37-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: offset RANGE frames (1P/1F, 2P/0F) group by peer values; RANGE + EXCLUDE GROUP composes (see harness).

## Observables

- `range_1_1` = `1,55|2,85|2,85|3,115|4,70`
- `range_2_0` = `1,10|2,55|2,55|3,85|4,115`
- `range_excl_group` = `1,45|2,40|2,40|3,85|4,30`
