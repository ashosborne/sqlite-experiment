# engine-lookaside35-002-C003 — run-11 oneshot characterization

Feature: `engine-lookaside35-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: 512x2 pool + six live stmts: USED positive, MISS_FULL grows, drain to 0 (see harness).

## Observables

- `used_live_pos` = `1`
- `miss_full_grew` = `1`
- `used_drained_zero` = `1`
