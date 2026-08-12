# engine-conn-003-C001 — run-11 oneshot characterization

Feature: `engine-conn-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: update_hook op/db/table/rowid for INSERT/UPDATE/DELETE (see harness).

## Observables

- `log` = `[18 main t 5] [23 main t 5] [9 main t 5]`
- `count` = `3`
