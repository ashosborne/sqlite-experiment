# engine-prepare-001-C001 — run-11 oneshot characterization

Feature: `engine-prepare-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: prepare kitchen SELECT; step rows; column_int/text; finalize (see harness).

## Observables

- `prepare.rc` = `0`
- `step1.rc` = `100`
- `a1` = `1`
- `b1` = `x`
- `step2.rc` = `100`
- `a2` = `2`
- `b2` = `y`
- `step3.rc` = `101`
- `finalize.rc` = `0`
