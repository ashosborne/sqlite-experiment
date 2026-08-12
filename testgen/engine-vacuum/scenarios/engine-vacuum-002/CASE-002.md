# engine-vacuum-002-C002 — run-11 oneshot characterization

Feature: `engine-vacuum-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: journal_mode + integrity after VACUUM (see harness).

## Observables

- `vac` = `rc=0 err=-`
- `mode` = `delete`
- `integ` = `ok`
