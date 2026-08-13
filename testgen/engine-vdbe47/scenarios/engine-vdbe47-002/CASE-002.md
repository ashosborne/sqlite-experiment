# engine-vdbe47-002-C002 — run-11 oneshot characterization

Feature: `engine-vdbe47-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: step/step/reset/step cycle: SQLITE_ROW+value, SQLITE_DONE, then ROW+value again (see harness).

## Observables

- `step_cycle` = `100,1 101 100,1`
