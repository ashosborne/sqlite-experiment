# engine-vdbe48-002-C003 — run-11 oneshot characterization

Feature: `engine-vdbe48-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: step walks the cursor row by row (100:10/100:20/100:30/101); reset rewinds and the scan replays (see harness).

## Observables

- `cycle` = `100:10/100:20/100:30/101 reset_step 100:10`
