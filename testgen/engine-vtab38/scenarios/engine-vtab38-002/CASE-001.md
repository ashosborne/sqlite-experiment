# engine-vtab38-002-C001 — run-11 oneshot characterization

Feature: `engine-vtab38-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_drop_modules keeps the named survivors; dropped names stop resolving (see harness).

## Observables

- `drop_keep_memvt_rc` = `0`
- `cvt_ser_dropped` = `rc=1 err=no such module: ser`
- `cvt_memvt_kept` = `rc=0 err=-`
