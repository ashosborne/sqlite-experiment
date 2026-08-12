# engine-upsert-expr-001-C010 — run-11 oneshot characterization

Feature: `engine-upsert-expr-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: conflict on NON-targeted constraint still aborts (rc 19) (see harness).

## Observables

- `other` = `rc=19 err=UNIQUE constraint failed: t.c`
- `targeted` = `rc=0 err=-`
- `after` = `1`
