# engine-harvest28-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest28-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: auth IGNORE column -> NULL in results; READ consulted (see harness).

## Observables

- `q` = `ann,~|bob,~`
- `read_consults` = `1`
