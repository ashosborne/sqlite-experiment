# engine-blob-001-C005 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: no such table (schema-qualified) / quoted no such column / cannot open view (see harness).

## Observables

- `no_table` = `rc=1 err=no such table: main.nope`
- `no_col` = `rc=1 err=no such column: "zz"`
- `view` = `rc=1 err=cannot open view: v`
