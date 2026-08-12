# engine-pragma34-001-C007 — run-11 oneshot characterization

Feature: `engine-pragma34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: freelist_count 0 fresh; quick_check reports the smuggled CHECK violation (see harness).

## Observables

- `freelist` = `0`
- `quick_check` = `CHECK constraint failed in u`
