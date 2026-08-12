# engine-serialize2-001-C001 — run-11 oneshot characterization

Feature: `engine-serialize2-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: populated serialize -> deserialize round trip (rows preserved) (see harness).

## Observables

- `ser.nonempty` = `1`
- `deser.rc` = `0`
- `a` = `1`
- `b` = `x`
- `a` = `2`
- `b` = `y`
