# engine-harvest36-006-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pending PRAGMA page_size applies at VACUUM (4096 until VACUUM, then 8192) (see harness).

## Observables

- `ps_before` = `4096`
- `ps_pending` = `4096`
- `vacuum` = `rc=0 err=-`
- `ps_after` = `8192`
