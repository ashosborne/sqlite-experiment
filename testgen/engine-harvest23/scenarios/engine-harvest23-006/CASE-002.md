# engine-harvest23-006-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: extended errcodes 2067/1299/275 on constraint paths (see harness).

## Observables

- `uniq` = `rc=19 err=UNIQUE constraint failed: t.a`
- `uniq_ext` = `2067`
- `notnull` = `rc=19 err=NOT NULL constraint failed: t.b`
- `nn_ext` = `1299`
- `check` = `rc=19 err=CHECK constraint failed: c>0`
- `ck_ext` = `275`
