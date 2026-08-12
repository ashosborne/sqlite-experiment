# engine-harvest23-001-C003 — run-11 oneshot characterization

Feature: `engine-harvest23-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: reset clears all; duplicate registration invoked once (see harness).

## Observables

- `a_calls` = `1`
- `b_calls` = `2`
- `a_after_dup` = `2`
