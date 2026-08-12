# engine-harvest36-006-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pending PRAGMA auto_vacuum applies at VACUUM (0 until VACUUM, then 1) (see harness).

## Observables

- `av_before` = `0`
- `av_pending` = `0`
- `vacuum` = `rc=0 err=-`
- `av_after` = `1`
