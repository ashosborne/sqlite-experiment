# engine-harvest37-002-C003 — run-11 oneshot characterization

Feature: `engine-harvest37-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SQLITE_IGNORE semantics: INSERT/UPDATE skipped, DELETE proceeds (truncate-opt only) (see harness).

## Observables

- `ins_ignored` = `rc=0 err=-`
- `count_after_ins_ignore` = `2`
- `upd_ignored` = `rc=0 err=-`
- `vals_after_upd_ignore` = `1|2`
- `del_ignored` = `rc=0 err=-`
- `count_after_del_ignore` = `0`
