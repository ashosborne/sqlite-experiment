# engine-harvest40-006-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DQS_DDL: double-quoted strings in CHECK accepted by default and when on; off errors with C's should-this-be-a-string-literal message (see harness).

## Observables

- `dqs_ddl_default` = `rc=0 err=-`
- `dqs_ddl_on` = `rc=0 err=-`
- `dqs_ddl_off` = `rc=1 err=no such column: "nosuchcol" - should this be a string literal in single-quotes?`
