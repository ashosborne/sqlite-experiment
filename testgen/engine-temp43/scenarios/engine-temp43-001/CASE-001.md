# engine-temp43-001-C001 — run-11 oneshot characterization

Feature: `engine-temp43-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: CREATE TEMP/TEMPORARY TABLE lands in schema temp: unqualified and temp.-qualified reads agree, sqlite_temp_master lists the objects, sqlite_master does not (see harness).

## Observables

- `ct_temp` = `rc=0 err=- LOG=`
- `ct_temporary` = `rc=0 err=- LOG=`
- `ins_temp` = `rc=0 err=- LOG=`
- `sel_temp` = `rows=7`
- `sel_qual` = `rows=7`
- `temp_master` = `rows=tt,table|tt2,table`
- `main_master` = `rows=shared`
