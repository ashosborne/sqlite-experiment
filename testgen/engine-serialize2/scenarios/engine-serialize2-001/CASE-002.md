# engine-serialize2-001-C002 — run-11 oneshot characterization

Feature: `engine-serialize2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: serialize round trip preserves all five storage classes (see harness).

## Observables

- `ser.nonempty` = `1`
- `ty` = `integer`
- `ty` = `real`
- `ty` = `text`
- `ty` = `blob`
- `ty` = `null`
