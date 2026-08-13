# engine-harvest41-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest41-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SAVEPOINT DENY is the generic rc 23 not authorized; the consult fires before the savepoint-exists check (see harness).

## Observables

- `sp_deny` = `rc=23 err=not authorized LOG=[32|BEGIN|s3|~|~]`
- `rel_deny_missing` = `rc=23 err=not authorized LOG=[32|RELEASE|s3|~|~]`
