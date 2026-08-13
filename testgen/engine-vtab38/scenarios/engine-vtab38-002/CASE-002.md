# engine-vtab38-002-C002 — run-11 oneshot characterization

Feature: `engine-vtab38-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: drop-all with a live instance: the instance still scans, new CREATE and DROP TABLE both fail no such module (see harness).

## Observables

- `drop_all_with_live_rc` = `0`
- `live_select_after_drop` = `1|2|3`
- `cvt_after_dropall` = `rc=1 err=no such module: ser`
- `drop_table_after_dropall` = `rc=1 err=no such module: ser`
