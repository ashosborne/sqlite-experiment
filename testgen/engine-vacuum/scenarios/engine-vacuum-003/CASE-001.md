# engine-vacuum-003-C001 — run-11 oneshot characterization

Feature: `engine-vacuum-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: VACUUM INTO creates target; source unchanged; target readable (see harness).

## Observables

- `into` = `rc=0 err=-`
- `src` = `1,one|2,two`
- `target` = `1,one|2,two`
- `integ` = `ok`
