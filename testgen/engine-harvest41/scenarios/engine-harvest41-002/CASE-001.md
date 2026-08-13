# engine-harvest41-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest41-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SAVEPOINT/RELEASE/ROLLBACK TO fire code 32 with s1 = BEGIN/RELEASE/ROLLBACK and s2 = the savepoint name; plain BEGIN/ROLLBACK stay code 22 (see harness).

## Observables

- `sp_open` = `rc=0 err=- LOG=[32|BEGIN|s1|~|~]`
- `sp_release` = `rc=0 err=- LOG=[32|RELEASE|s1|~|~]`
- `sp_rbto` = `rc=0 err=- LOG=[32|BEGIN|s2|~|~][32|ROLLBACK|s2|~|~][32|RELEASE|s2|~|~]`
- `txn_rollback` = `rc=0 err=- LOG=[22|BEGIN|~|~|~][22|ROLLBACK|~|~|~]`
