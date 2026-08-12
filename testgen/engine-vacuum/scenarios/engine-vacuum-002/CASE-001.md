# engine-vacuum-002-C001 — run-11 oneshot characterization

Feature: `engine-vacuum-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: file rows survive VACUUM + reopen; integrity ok (see harness).

## Observables

- `vac` = `rc=0 err=-`
- `reopen` = `7|8`
- `integ` = `ok`
