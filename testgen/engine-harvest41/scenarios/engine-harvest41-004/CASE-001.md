# engine-harvest41-004-C001 — run-11 oneshot characterization

Feature: `engine-harvest41-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ALTER TABLE fires code 26 with INVERTED args (s1 = database, s2 = table) for RENAME and ADD COLUMN - outer event only (see harness).

## Observables

- `alter_rename` = `rc=0 err=- LOG=[26|main|t|~|~]`
- `after_rename` = `rows=u LOG=`
- `alter_addcol` = `rc=0 err=- LOG=[26|main|u|~|~]`
