# engine-collation-002-C001 — run-11 oneshot characterization

Feature: `engine-collation-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: column COLLATE clause drives WHERE + ORDER BY (see harness).

## Observables

- `where_decl` = `cat`
- `order_decl` = `APE|cat|Dog`
