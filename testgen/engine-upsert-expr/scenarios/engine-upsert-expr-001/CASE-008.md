# engine-upsert-expr-001-C008 — run-11 oneshot characterization

Feature: `engine-upsert-expr-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: OR IGNORE / OR REPLACE via expression unique key (see harness).

## Observables

- `ignore` = `rc=0 err=-`
- `mid` = `Delta,1`
- `replace` = `rc=0 err=-`
- `after` = `DELTA,3`
