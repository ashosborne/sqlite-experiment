# engine-harvest42-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest42-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: TWO seed terms interleave in FIFO order (1|10|2|11|3|12|13) - the extracted row IS the recursive table, outputs enqueue (see harness).

## Observables

- `r_two_seed` = `rows=1|10|2|11|3|12|13 LOG=`
