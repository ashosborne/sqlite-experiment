# backup-api-003-C001 — run-11 oneshot characterization

Feature: `backup-api-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: single-threaded source-write between steps; rc sequence + counters only (see harness).

## Observables

- `init.nonnull` = `1`
- `step1.rc` = `0`
- `remaining.after_step1` = `1`
- `pagecount.after_step1` = `2`
- `step_rest.rc` = `101`
- `finish.rc` = `0`
