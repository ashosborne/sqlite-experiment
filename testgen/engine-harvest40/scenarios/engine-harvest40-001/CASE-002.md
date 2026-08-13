# engine-harvest40-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest40-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: renaming a vtab whose module lacks xRename still succeeds (no callback) (see harness).

## Observables

- `create2` = `rc=0 err=- mlog=`
- `rename2` = `rc=0 err=- mlog=`
- `master2` = `nw|wt`
