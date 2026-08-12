# engine-prepare-001-C003 — run-11 oneshot characterization

Feature: `engine-prepare-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pzTail across two statements; both execute (see harness).

## Observables

- `prepare.rc` = `0`
- `tail` = ` SELECT 2`
- `step.rc` = `100`
- `v1` = `1`
- `prepare2.rc` = `0`
- `step2.rc` = `100`
- `v2` = `2`
