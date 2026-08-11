# backup-api-001-C001 — run-11 oneshot characterization

Feature: `backup-api-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: backup init/step(-1)/finish on empty :memory: pair (see harness).

## Observables

- `init.nonnull` = `1`
- `step_all.rc` = `101`
- `finish.rc` = `0`
