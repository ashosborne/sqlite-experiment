# engine-harvest43-001-C003 — run-11 oneshot characterization

Feature: `engine-harvest43-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: json_patch replaces arrays wholesale; NULL document yields NULL; chained set-then-remove composes (see harness).

## Observables

- `patch_arr` = `rows={"a":[9]}`
- `null_json` = `rows=~`
- `chain` = `rows={"a":[0,2]}`
