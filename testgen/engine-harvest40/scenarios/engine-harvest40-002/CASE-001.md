# engine-harvest40-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: explicit txn: xBegin at first write, xSavepoint 0/1 per SAVEPOINT, xRollbackTo/xRelease with the savepoint's index, xSync+xCommit at COMMIT; module storage tracks visibility (see harness).

## Observables

- `create` = `rc=0 err=- mlog=[sync][commit]`
- `begin` = `rc=0 err=- mlog=`
- `ins1` = `rc=0 err=- mlog=[begin][ins:11]`
- `sp1` = `rc=0 err=- mlog=[svpt:0]`
- `ins2` = `rc=0 err=- mlog=[ins:22]`
- `sp2` = `rc=0 err=- mlog=[svpt:1]`
- `ins3` = `rc=0 err=- mlog=[ins:33]`
- `pre_rb` = `11|22|33`
- `rbto1` = `rc=0 err=- mlog=[rbto:0]`
- `post_rb` = `11`
- `rel1` = `rc=0 err=- mlog=[release:0]`
- `commit` = `rc=0 err=- mlog=[sync][commit]`
- `final` = `11`
