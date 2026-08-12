# engine-vtab31-001-C010 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: drop + recreate same name with different arg -> new shape/values (see harness).

## Observables

- `before` = `1|2`
- `after` = `1|2|3|4|5|6`
