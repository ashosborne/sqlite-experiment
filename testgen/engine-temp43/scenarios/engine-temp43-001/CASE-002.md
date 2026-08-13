# engine-temp43-001-C002 — run-11 oneshot characterization

Feature: `engine-temp43-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: a TEMP table shadows a same-named main table for unqualified access; unqualified DROP removes the TEMP one first; temp.-qualified DROP works (see harness).

## Observables

- `shadow` = `rc=0 err=- LOG=`
- `ins_shadow` = `rc=0 err=- LOG=`
- `resolve_unqual` = `rows=temp`
- `resolve_main` = `rows=main`
- `resolve_temp` = `rows=temp`
- `drop_unqual` = `rc=0 err=- LOG=`
- `after_drop_unqual` = `rows=main`
- `after_drop_temp_master` = `rows=tt|tt2`
- `drop_qual_temp` = `rc=0 err=- LOG=`
- `temp_master2` = `rows=tt`
