# engine-harvest40-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ALTER RENAME of a vtab fires xRename with the new name; sqlite_master sql rewritten with the quoted new name; old name gone (see harness).

## Observables

- `create` = `rc=0 err=- mlog=[sync][commit]`
- `ins` = `rc=0 err=- mlog=[begin][ins:5][sync][commit]`
- `rename` = `rc=0 err=- mlog=[rename:wt]`
- `master` = `wt,CREATE VIRTUAL TABLE "wt" USING ser`
- `sel_new` = `5`
- `sel_old` = `rc=1 err=no such table: vt mlog=`
