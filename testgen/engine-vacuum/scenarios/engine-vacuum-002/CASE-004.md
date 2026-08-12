# engine-vacuum-002-C004 — run-11 oneshot characterization

Feature: `engine-vacuum-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: unique index enforced after VACUUM + reopen (see harness).

## Observables

- `vac` = `rc=0 err=-`
- `probe` = `2`
- `dup` = `rc=19 err=UNIQUE constraint failed: t.a`
