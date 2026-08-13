# engine-harvest37-001-C005 — run-11 oneshot characterization

Feature: `engine-harvest37-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_uri_parameter/int64/boolean over the opened URI; db_filename; :memory: filename empty string (see harness).

## Observables

- `db_filename` = `/tmp/h37u.db`
- `uri_param_foo` = `bar`
- `uri_param_missing` = `(null)`
- `uri_int_baz` = `7`
- `uri_bool_foo` = `0`
- `uri_bool_missing_dflt` = `1`
- `memory_filename` = `''`
