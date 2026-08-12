# engine-conn-003-C008 — run-11 oneshot characterization

Feature: `engine-conn-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: trace_v2 CLOSE fires once at close (see harness).

## Observables

- `pre_close` = `0`
- `post_close` = `1`
