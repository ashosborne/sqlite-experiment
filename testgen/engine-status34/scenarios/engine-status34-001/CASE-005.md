# engine-status34-001-C005 — run-11 oneshot characterization

Feature: `engine-status34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: resetFlag pulls MALLOC_COUNT highwater down to current (see harness).

## Observables

- `reset_hi_eq_cur` = `1`
- `hi_le_prev` = `1`
