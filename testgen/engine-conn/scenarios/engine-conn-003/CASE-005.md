# engine-conn-003-C005 — run-11 oneshot characterization

Feature: `engine-conn-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: commit_hook replacement returns prior arg; only new hook fires (see harness).

## Observables

- `old_is_arg` = `1`
- `first_calls` = `0`
- `second_calls` = `1`
