# engine-collation-001-C005 — run-11 oneshot characterization

Feature: `engine-collation-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: create_collation_v2 xDestroy on replace and close (see harness).

## Observables

- `after_reg` = `0`
- `after_replace` = `1`
- `after_close` = `2`
