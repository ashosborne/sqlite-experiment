# engine-harvest23-004-C004 — run-11 oneshot characterization

Feature: `engine-harvest23-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ntile(2)/ntile(3) bucket assignment (see harness).

## Observables

- `ntile2` = `5,1|10,1|15,1|20,2|30,2`
- `ntile3` = `5,1|10,1|15,2|20,2|30,3`
