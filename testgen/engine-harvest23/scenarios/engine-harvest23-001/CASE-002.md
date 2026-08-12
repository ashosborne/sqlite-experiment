# engine-harvest23-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: cancel one entry (rc 1 / repeat 0); other still runs (see harness).

## Observables

- `cancel_a` = `1`
- `cancel_a_again` = `0`
- `a_calls` = `1`
- `b_calls` = `2`
