# engine-temp43-002-C001 — run-11 oneshot characterization

Feature: `engine-temp43-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: CREATE TEMP TABLE fires authorizer code 4 and dropping one fires 13, both (s1 = table, s3 = temp) - outer events only, no sqlite_temp_master tail frozen (see harness).

## Observables

- `a_ct_temp` = `rc=0 err=- LOG=[4|at|~|temp|~]`
- `a_drop_temp` = `rc=0 err=- LOG=[13|at|~|temp|~]`
