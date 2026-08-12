# engine-conn-001-C001 — run-11 oneshot characterization

Feature: `engine-conn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: clean close OK; NULL close/close_v2 no-ops (see harness).

## Observables

- `close` = `0`
- `close_null` = `0`
- `close_v2_null` = `0`
