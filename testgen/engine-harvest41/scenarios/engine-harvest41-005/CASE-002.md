# engine-harvest41-005-C002 — run-11 oneshot characterization

Feature: `engine-harvest41-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: CREATE VIRTUAL TABLE fires 29 and dropping a vtab fires 30, both with s1 = table, s2 = module, s3 = main - outer only (see harness).

## Observables

- `cv` = `rc=0 err=- LOG=[29|vt|ser|main|~]`
- `dv` = `rc=0 err=- LOG=[30|vt|ser|main|~]`
