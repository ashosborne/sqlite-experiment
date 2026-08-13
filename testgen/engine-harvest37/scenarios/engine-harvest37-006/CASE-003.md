# engine-harvest37-006-C003 — run-11 oneshot characterization

Feature: `engine-harvest37-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: db_config DQS_DML: default on accepts double-quoted strings, off errors with C hint; DEFENSIVE toggle rc; bad op (see harness).

## Observables

- `dqs_dml_get_rc` = `0`
- `dqs_dml_default` = `1`
- `dqs_on_ins` = `rc=0 err=-`
- `dqs_off_ins` = `rc=1 err=no such column: "notacol" - should this be a string literal in single-quotes?`
- `defensive_rc` = `0`
- `badop_rc` = `1`
