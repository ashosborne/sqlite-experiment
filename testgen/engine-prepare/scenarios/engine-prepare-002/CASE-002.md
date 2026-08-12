# engine-prepare-002-C002 — run-11 oneshot characterization

Feature: `engine-prepare-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: typed binds null/int/int64/double/text/blob -> typeof row (see harness).

## Observables

- `b.null` = `0`
- `b.int` = `0`
- `b.int64` = `0`
- `b.double` = `0`
- `b.text` = `0`
- `b.blob` = `0`
- `step.rc` = `101`
- `ty0` = `null`
- `ty1` = `integer`
- `ty2` = `integer`
- `ty3` = `real`
- `ty4` = `text`
- `ty5` = `blob`
