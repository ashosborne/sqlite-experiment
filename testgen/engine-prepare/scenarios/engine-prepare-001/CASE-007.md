# engine-prepare-001-C007 — run-11 oneshot characterization

Feature: `engine-prepare-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: autoreset: step after DONE re-runs (OMIT_AUTORESET=off) (see harness).

## Observables

- `step1.rc` = `100`
- `step2.rc` = `101`
- `step3.rc` = `100`
- `a` = `9`
