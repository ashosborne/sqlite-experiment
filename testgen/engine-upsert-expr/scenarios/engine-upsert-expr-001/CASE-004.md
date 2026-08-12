# engine-upsert-expr-001-C004 — run-11 oneshot characterization

Feature: `engine-upsert-expr-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: multi-expression index (lower(a), b) target (see harness).

## Observables

- `dup` = `rc=0 err=-`
- `nodup` = `rc=0 err=-`
- `after` = `Kilo,5|KILO,6`
