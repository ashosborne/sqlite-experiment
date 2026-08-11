# prepare-statement-api-004-C001 — run-11 oneshot characterization

Feature: `prepare-statement-api-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: column coercion: column_type then column_int on text '42abc' (see harness).

## Observables

- `prepare.rc` = `0`
- `column_type.before` = `3`
- `column_int.coerced` = `42`
