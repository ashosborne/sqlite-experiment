# engine-harvest40-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest40-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: late join under open savepoints: catch-up xSavepoint(0) after xBegin, txn-savepoint excluded from numbering, ROLLBACK TO below the join passes -1, RELEASE of the txn savepoint commits (xSync+xCommit, no xRelease) (see harness).

## Observables

- `create` = `rc=0 err=- mlog=[sync][commit]`
- `sp_a` = `rc=0 err=- mlog=`
- `sp_b` = `rc=0 err=- mlog=`
- `ins` = `rc=0 err=- mlog=[begin][svpt:0][ins:7]`
- `sp_c` = `rc=0 err=- mlog=[svpt:1]`
- `ins2` = `rc=0 err=- mlog=[ins:8]`
- `rb_c` = `rc=0 err=- mlog=[rbto:1]`
- `after_rb_c` = `7`
- `rb_a` = `rc=0 err=- mlog=[rbto:-1]`
- `after_rb_a` = ``
- `rel_a` = `rc=0 err=- mlog=[sync][commit]`
- `final` = ``
